use anyhow::Result;
use std::collections::BTreeMap;
use std::fs;
use walkdir::WalkDir;

use crate::tests_writer::{codegen, utility::{self, Config, ExtractedTest}};

pub fn run(cfg: Config) -> Result<()> {
    // Ensure destination directory exists
    fs::create_dir_all(&cfg.dst)?;
    let (include_set, exclude_set) = codegen::build_globs(&cfg.include, &cfg.exclude)?;

    // Collect tests grouped by source filename stem
    let mut grouped: BTreeMap<String, Vec<ExtractedTest>> = BTreeMap::new();

    for entry in WalkDir::new(&cfg.src).into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() { continue; }
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("cs") { continue; }
        let rel = pathdiff::diff_paths(path, &cfg.src).unwrap_or_else(|| path.to_path_buf());
        if !codegen::is_included(&include_set, &exclude_set, &rel) { continue; }

        let mut content = match fs::read_to_string(path) { Ok(s) => s, Err(_) => continue };
        if content.starts_with('\u{FEFF}') {
            // Strip UTF-8 BOM if present
            if let Some(stripped) = content.strip_prefix('\u{FEFF}') { content = stripped.to_string(); }
        }

        if cfg.skip_overrides && codegen::file_overrides_parse_context(&content) { continue; }

        let methods = utility::collect_test_methods(&content);
        let mut tests = codegen::extract_tests(&content, &methods, cfg.skip_diagnostics);
        // Keep only tests that parse successfully with our parser under the selected wrappers
        tests.retain(|t| utility::prevalidate(t));
        if tests.is_empty() { continue; }
        if tests.len() > cfg.max_per_file { tests.truncate(cfg.max_per_file); }

        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("tests").to_string();
        grouped.entry(stem).or_default().extend(tests);
    }

    // Write out groups
    if grouped.is_empty() {
        eprintln!("No tests matched. Check --src/--include/--exclude.");
    }

    // Rewrite mod.rs with current set of modules
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
    for m in modules { mod_contents.push_str(&format!("mod {};\n", m)); }
    fs::write(&mod_rs_path, mod_contents)?;

    Ok(())
}
