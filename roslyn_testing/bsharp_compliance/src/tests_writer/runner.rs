use anyhow::Result;
use std::collections::BTreeMap;
use std::fs;
use walkdir::WalkDir;

use crate::generator::structure_dsl;
use crate::generator::{emitter, model as new_model, pipeline, writer};
use crate::tests_writer::{
    codegen,
    utility::{self, Config, ExtractedTest},
};

pub fn run(cfg: Config) -> Result<()> {
    // Ensure destination directory exists
    fs::create_dir_all(&cfg.dst)?;
    let (include_set, exclude_set) = codegen::build_globs(&cfg.include, &cfg.exclude)?;

    if cfg.structure_mode {
        let verbose = std::env::var("BSHARP_LOG").is_ok();
        eprintln!("[structure] start: src={:?} dst={:?}", cfg.src, cfg.dst);
        eprintln!(
            "[structure] include={:?} exclude={:?}",
            cfg.include, cfg.exclude
        );
        // Structure mode: extract UsingTree + structure DSL blocks and emit into generated/parsing/
        let parsing_dir = cfg.dst.join("parsing");
        fs::create_dir_all(&parsing_dir)?;
        writer::ensure_parent_mod_has_submod(&cfg.dst, "parsing").ok();

        let mut grouped: BTreeMap<String, Vec<structure_dsl::ExtractedStructureTest>> =
            BTreeMap::new();
        let mut visited = 0usize;
        let mut matched = 0usize;
        for entry in WalkDir::new(&cfg.src).into_iter().filter_map(|e| e.ok()) {
            if !entry.file_type().is_file() {
                continue;
            }
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("cs") {
                continue;
            }
            let rel = pathdiff::diff_paths(path, &cfg.src).unwrap_or_else(|| path.to_path_buf());
            visited += 1;
            if !codegen::is_included(&include_set, &exclude_set, &rel) {
                if verbose {
                    eprintln!("[structure] skip: {:?}", rel);
                }
                continue;
            }
            matched += 1;
            if verbose {
                eprintln!("[structure] file: {:?}", rel);
            }

            let mut content = match fs::read_to_string(path) {
                Ok(s) => s,
                Err(_) => continue,
            };
            if content.starts_with('\u{FEFF}') {
                if let Some(stripped) = content.strip_prefix('\u{FEFF}') {
                    content = stripped.to_string();
                }
            }

            let methods = utility::collect_test_methods(&content);
            let mut cases = structure_dsl::extract_structure_tests(&content);
            if verbose {
                eprintln!("[structure] cases found: {}", cases.len());
            }
            for c in &mut cases {
                c.method_name = utility::find_enclosing_method_name(&methods, c.call_pos);
            }
            if cases.is_empty() {
                continue;
            }
            if cases.len() > cfg.max_per_file {
                eprintln!(
                    "[structure] truncating cases: {} -> {}",
                    cases.len(),
                    cfg.max_per_file
                );
                cases.truncate(cfg.max_per_file);
            }

            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("tests")
                .to_string();
            eprintln!("[structure] add group: {} ({} cases)", stem, cases.len());
            grouped.entry(stem).or_default().extend(cases);
        }
        eprintln!(
            "[structure] visited .cs files: {}, matched include: {}",
            visited, matched
        );

        for (stem, cases) in grouped {
            let module_name = codegen::sanitize_mod_name(&stem);
            eprintln!(
                "[structure] emit module: {} (stem: {}, cases: {})",
                module_name,
                stem,
                cases.len()
            );
            let contents = crate::generator::structure_emitter::emit_structure_tests_for_group(
                &module_name,
                &stem,
                &cases,
            )?;
            crate::generator::writer::write_group_file(
                &parsing_dir,
                &module_name,
                &stem,
                &contents,
            )?;
            eprintln!("[structure] wrote generated/parsing/{}.rs", module_name);
        }

        return Ok(());
    }

    // Collect tests grouped by source filename stem (legacy and new_emitter paths)
    let mut grouped: BTreeMap<String, Vec<ExtractedTest>> = BTreeMap::new();

    for entry in WalkDir::new(&cfg.src).into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("cs") {
            continue;
        }
        let rel = pathdiff::diff_paths(path, &cfg.src).unwrap_or_else(|| path.to_path_buf());
        if !codegen::is_included(&include_set, &exclude_set, &rel) {
            continue;
        }

        let mut content = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => continue,
        };
        if content.starts_with('\u{FEFF}') {
            // Strip UTF-8 BOM if present
            if let Some(stripped) = content.strip_prefix('\u{FEFF}') {
                content = stripped.to_string();
            }
        }

        if cfg.skip_overrides && codegen::file_overrides_parse_context(&content) {
            continue;
        }

        let methods = utility::collect_test_methods(&content);
        let mut tests = pipeline::extract_tests_facade(&content, &methods, cfg.skip_diagnostics);
        // Optionally filter to tests that our parser currently accepts under the selected wrappers
        if cfg.prevalidate {
            tests.retain(utility::prevalidate);
        }
        if tests.is_empty() {
            continue;
        }
        if tests.len() > cfg.max_per_file {
            tests.truncate(cfg.max_per_file);
        }

        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("tests")
            .to_string();
        grouped.entry(stem).or_default().extend(tests);
    }

    // Write out groups
    if grouped.is_empty() {
        eprintln!("No tests matched. Check --src/--include/--exclude.");
    }

    if cfg.use_new_emitter {
        for (stem, tests) in grouped {
            let module_name = codegen::sanitize_mod_name(&stem);
            // Map to new model ExtractedTest
            let mapped: Vec<new_model::ExtractedTest> =
                tests.into_iter().map(|t| map_old_to_new(&t)).collect();
            let contents = emitter::emit_tests_for_group(&module_name, &stem, &mapped)?;
            writer::write_group_file(&cfg.dst, &module_name, &stem, &contents)?;
        }
    } else {
        // Legacy path preserves existing behavior and single mod.rs rewrite
        let mod_rs_path = cfg.dst.join("mod.rs");
        let mut modules: Vec<String> = Vec::new();
        for (stem, tests) in grouped {
            let module_name = codegen::sanitize_mod_name(&stem);
            codegen::write_group(&cfg.dst, &module_name, &stem, &tests)?;
            modules.push(module_name);
        }
        modules.sort();
        modules.dedup();
        let mut mod_contents = String::from("// generated tests\n");
        for m in modules {
            mod_contents.push_str(&format!("mod {};\n", m));
        }
        fs::write(&mod_rs_path, mod_contents)?;
    }

    Ok(())
}

fn map_old_to_new(t: &ExtractedTest) -> new_model::ExtractedTest {
    let expected = t
        .expected_diag_count
        .map(|n| new_model::ExpectedDiagnostics {
            count: n,
            items: vec![],
        });
    new_model::ExtractedTest {
        category: match t.category {
            utility::Category::Tree => new_model::TestCategory::Tree,
            utility::Category::Statement => new_model::TestCategory::Statement,
            utility::Category::Declaration => new_model::TestCategory::Declaration,
            utility::Category::Expression => new_model::TestCategory::Expression,
        },
        method_name: t.method_name.clone(),
        code: t.code.clone(),
        expected,
        options: new_model::TestOptions::default(),
    }
}
