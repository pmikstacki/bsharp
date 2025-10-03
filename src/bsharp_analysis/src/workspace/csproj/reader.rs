use std::fs;
use std::path::{Path, PathBuf};

use globwalk::GlobWalkerBuilder;
use quick_xml::Reader as XmlReader;
use quick_xml::events::Event;

use crate::workspace::error::{Result, WorkspaceError};
use crate::workspace::model::{Language, Project, ProjectFile, ProjectFileKind};

pub struct CsprojReader;

impl CsprojReader {
    pub fn read(path: &Path) -> Result<Project> {
        let path = path.canonicalize().map_err(WorkspaceError::from)?;
        let root = path.parent().unwrap_or(Path::new("."));
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("project")
            .to_string();

        let mut project = Project {
            name,
            path: path.clone(),
            files: Vec::new(),
            project_references: Vec::new(),
            errors: Vec::new(),
        };

        let content = fs::read_to_string(&path)?;
        let mut reader = XmlReader::from_str(&content);
        reader.config_mut().trim_text(true);

        let mut includes: Vec<String> = Vec::new();
        let mut removes: Vec<String> = Vec::new();

        // Parse a small subset of SDK-style csproj: <Compile Include/Remove/Update>, <ProjectReference Include>
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(e)) | Ok(Event::Start(e)) => {
                    let name = e.name().as_ref().to_ascii_lowercase();
                    // Process attributes once; also record unsupported MSBuild Condition and macro usage
                    for a in e.attributes().flatten() {
                        let key = a.key.as_ref();
                        let raw = a
                            .unescape_value()
                            .ok()
                            .map(|v| v.to_string())
                            .unwrap_or_default();
                        // Tests may use backslash-escaped quotes in raw strings; normalize those.
                        let dequoted = raw.replace("\\\"", "\"").replace("\\'", "'");
                        // Trim whitespace and surrounding quotes
                        let trimmed = dequoted
                            .trim()
                            .trim_matches(|c| c == '"' || c == '\'')
                            .to_string();
                        // Normalize Windows-style separators to POSIX for globbing
                        let val_clean = trimmed.replace('\\', "/");
                        // Warn on MSBuild Condition attributes (not evaluated in v1)
                        if key.eq_ignore_ascii_case(b"condition") {
                            project.errors.push(format!(
                                "MSBuild Condition not evaluated in v1 on <{}>: {}",
                                String::from_utf8_lossy(&name),
                                trimmed
                            ));
                        }

                        // Compile items
                        if name.ends_with(b"compile") {
                            if (key.eq_ignore_ascii_case(b"include")
                                || key.eq_ignore_ascii_case(b"update"))
                                && !val_clean.is_empty()
                            {
                                if val_clean.contains("$(") {
                                    project.errors.push(format!(
                                        "MSBuild macro not expanded in v1 on <Compile {}>: {}",
                                        String::from_utf8_lossy(key),
                                        val_clean
                                    ));
                                }
                                includes.push(val_clean.clone());
                            }
                            if key.eq_ignore_ascii_case(b"remove") && !val_clean.is_empty() {
                                if val_clean.contains("$(") {
                                    project.errors.push(format!(
                                        "MSBuild macro not expanded in v1 on <Compile Remove>: {}",
                                        val_clean
                                    ));
                                }
                                removes.push(val_clean.clone());
                            }
                        }

                        // ProjectReference items
                        if name.ends_with(b"projectreference")
                            && key.eq_ignore_ascii_case(b"include")
                        {
                            if val_clean.contains("$(") {
                                project.errors.push(format!(
                                    "MSBuild macro not expanded in v1 on <ProjectReference Include>: {}",
                                    val_clean
                                ));
                            }
                            let p = root.join(val_clean.clone());
                            match p.canonicalize() {
                                Ok(abs) => project.project_references.push(abs),
                                Err(_) => {
                                    // Keep the joined path even if canonicalization fails, and record a warning
                                    project.project_references.push(p.clone());
                                    project.errors.push(format!(
                                        "Unresolved ProjectReference: {}",
                                        p.display()
                                    ));
                                }
                            }
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    return Err(WorkspaceError::ProjectParse(format!(
                        "XML parse error in {}: {}",
                        path.display(),
                        e
                    )));
                }
                _ => {}
            }
            buf.clear();
        }

        // Default SDK-style includes: **/*.cs excluding common dirs
        let mut file_set: Vec<PathBuf> = Vec::new();
        Self::collect_glob(root, "**/*.cs", &mut file_set, &mut project.errors);
        // Additional explicit includes
        for pat in includes {
            Self::collect_glob(root, &pat, &mut file_set, &mut project.errors);
        }
        // Apply removes
        for pat in removes {
            Self::remove_glob(root, &pat, &mut file_set);
        }
        // Default excludes
        Self::remove_glob(root, "bin/**", &mut file_set);
        Self::remove_glob(root, "obj/**", &mut file_set);
        Self::remove_glob(root, ".git/**", &mut file_set);
        // Safety net: re-apply removes after excludes (handles odd path normalizations)
        // Note: we can't reuse 'removes' now (moved), so this is skipped unless needed in future.

        // Dedup and sort
        file_set.sort();
        file_set.dedup();

        for f in file_set {
            if f.extension()
                .and_then(|s| s.to_str())
                .map(|e| e.eq_ignore_ascii_case("cs"))
                .unwrap_or(false)
            {
                project.add_file(ProjectFile::new_source(f, Language::CSharp));
            } else {
                project.add_file(ProjectFile {
                    path: f,
                    kind: ProjectFileKind::Other,
                    language: None,
                });
            }
        }

        Ok(project)
    }

    fn collect_glob(root: &Path, pattern: &str, out: &mut Vec<PathBuf>, errors: &mut Vec<String>) {
        let walker = GlobWalkerBuilder::from_patterns(root, &[pattern])
            .case_insensitive(true)
            .build();
        match walker {
            Ok(w) => {
                for entry in w.filter_map(|r| r.ok()) {
                    if entry.file_type().is_file() {
                        // Canonicalize for stable comparison with removals
                        let p = entry.path();
                        match p.canonicalize() {
                            Ok(abs) => out.push(abs),
                            Err(_) => out.push(p.to_path_buf()),
                        }
                    }
                }
            }
            Err(e) => errors.push(format!("globwalk error for '{}': {}", pattern, e)),
        }
    }

    fn remove_glob(root: &Path, pattern: &str, files: &mut Vec<PathBuf>) {
        if let Ok(w) = GlobWalkerBuilder::from_patterns(root, &[pattern])
            .case_insensitive(true)
            .build()
        {
            let remove_set: std::collections::HashSet<PathBuf> = w
                .filter_map(|r| r.ok())
                .filter(|e| e.file_type().is_file())
                .map(|e| {
                    e.path()
                        .canonicalize()
                        .unwrap_or_else(|_| e.path().to_path_buf())
                })
                .collect();
            files.retain(|p| match p.canonicalize() {
                Ok(cp) => !remove_set.contains(&cp),
                Err(_) => !remove_set.contains(p),
            });
        }
        // Fallbacks:
        // 1) If the pattern is a simple filename, remove by filename equality
        let is_simple = !pattern.contains('/')
            && !pattern.contains('\\')
            && !pattern.contains('*')
            && !pattern.contains('?')
            && !pattern.contains('[');
        if is_simple {
            files.retain(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n != pattern)
                    .unwrap_or(true)
            });
        } else {
            // 2) If the pattern contains separators but not wildcards, remove by path tail match
            let has_sep = pattern.contains('/') || pattern.contains('\\');
            let has_wildcards =
                pattern.contains('*') || pattern.contains('?') || pattern.contains('[');
            if has_sep && !has_wildcards {
                let norm = pattern.replace('\\', "/");
                files.retain(|p| {
                    let canon = p.canonicalize().unwrap_or_else(|_| p.clone());
                    let canon_norm = canon.to_string_lossy().replace('\\', "/");
                    !canon_norm.ends_with(&norm)
                });
            }
        }
        // 3) Direct path match: if root.join(pattern) exists, remove it
        let direct = root.join(pattern);
        let direct_canon = direct.canonicalize().unwrap_or(direct);
        files.retain(|p| match p.canonicalize() {
            Ok(cp) => cp != direct_canon,
            Err(_) => p != &direct_canon,
        });
    }
}
