use std::fs;
use std::path::Path;

use crate::workspace::error::{Result, WorkspaceError};
use crate::workspace::model::{ProjectRef, Solution};

pub struct SolutionReader;

impl SolutionReader {
    pub fn read(path: &Path) -> Result<Solution> {
        let path = path.canonicalize().map_err(WorkspaceError::from)?;
        let content = fs::read_to_string(&path)?;
        let parsed = solp::parse_str(&content).map_err(|e| WorkspaceError::SolutionParse(e.to_string()))?;

        let root = path.parent().unwrap_or(Path::new("."));
        let mut refs: Vec<ProjectRef> = Vec::new();
        let mut errors: Vec<String> = Vec::new();

        for p in parsed.projects {
            // Prefer filtering by extension to keep v1 simple
            // Normalize possible Windows-style separators to POSIX for cross-platform tests
            let rel_norm = p.path_or_uri.replace('\\', "/");
            let rel = Path::new(&rel_norm);
            if rel.extension().and_then(|e| e.to_str()).map(|s| s.eq_ignore_ascii_case("csproj")).unwrap_or(false) {
                let abs = root.join(rel);
                match abs.canonicalize() {
                    Ok(abs_path) => {
                        refs.push(ProjectRef { name: p.name.to_string(), path: abs_path });
                    }
                    Err(_) => {
                        errors.push(format!("Project path not found: {}", abs.display()))
                    }
                }
            }
        }

        // Stable ordering
        refs.sort_by(|a, b| a.path.cmp(&b.path));

        Ok(Solution { path, projects: refs, errors })
    }
}
