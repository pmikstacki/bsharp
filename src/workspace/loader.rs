use std::fs;
use std::path::{Path, PathBuf};
use std::collections::{HashSet, VecDeque};

use crate::workspace::csproj::CsprojReader;
use crate::workspace::error::{Result, WorkspaceError};
use crate::workspace::model::{Project, Workspace};
use crate::workspace::sln::SolutionReader;
use crate::workspace::source_map::SourceMap;

pub struct WorkspaceLoader;

#[derive(Debug, Clone, Copy)]
pub struct WorkspaceLoadOptions {
    pub follow_refs: bool,
}

impl Default for WorkspaceLoadOptions {
    fn default() -> Self { Self { follow_refs: true } }
}

impl WorkspaceLoader {
    pub fn from_path(path: &Path) -> Result<Workspace> {
        Self::from_path_with_options(path, WorkspaceLoadOptions::default())
    }

    pub fn from_path_with_options(path: &Path, opts: WorkspaceLoadOptions) -> Result<Workspace> {
        let meta = fs::metadata(path)?;
        if meta.is_file() {
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
            match ext.as_str() {
                "sln" => Self::from_sln_with_options(path, opts),
                "csproj" => Self::from_csproj_with_options(path, opts),
                _ => Err(WorkspaceError::Unsupported(path.display().to_string())),
            }
        } else if meta.is_dir() {
            Self::from_dir_with_options(path, opts)
        } else {
            Err(WorkspaceError::InvalidPath(path.display().to_string()))
        }
    }

    fn from_sln(path: &Path) -> Result<Workspace> { Self::from_sln_with_options(path, WorkspaceLoadOptions::default()) }

    fn from_sln_with_options(path: &Path, opts: WorkspaceLoadOptions) -> Result<Workspace> {
        let solution = SolutionReader::read(path)?;
        let root = solution.path.parent().unwrap_or(Path::new(".")).to_path_buf();
        let mut projects: Vec<Project> = Vec::new();
        for p in &solution.projects {
            match CsprojReader::read(&p.path) {
                Ok(prj) => projects.push(prj),
                Err(e) => {
                    // Record a stub project with error so we don't fail the whole workspace
                    let mut prj = Project { name: p.name.clone(), path: p.path.clone(), ..Default::default() };
                    prj.errors.push(format!("{}", e));
                    projects.push(prj);
                }
            }
        }
        // Follow ProjectReference transitively within root if enabled
        if opts.follow_refs {
            Self::follow_project_references(&root, &mut projects);
        }
        // Deterministic ordering
        projects.sort_by(|a, b| a.path.cmp(&b.path));
        let source_paths = projects.iter().flat_map(|p| p.files.iter().map(|f| f.path.clone()));
        let source_map = SourceMap::from_paths(source_paths);
        Ok(Workspace { root, projects, solution: Some(solution), source_map })
    }

    fn from_csproj(path: &Path) -> Result<Workspace> { Self::from_csproj_with_options(path, WorkspaceLoadOptions::default()) }

    fn from_csproj_with_options(path: &Path, opts: WorkspaceLoadOptions) -> Result<Workspace> {
        let prj = CsprojReader::read(path)?;
        let root = prj.path.parent().unwrap_or(Path::new(".")).to_path_buf();
        let mut projects = vec![prj];
        // Follow ProjectReference transitively within root if enabled
        if opts.follow_refs {
            Self::follow_project_references(&root, &mut projects);
        }
        // Deterministic ordering
        projects.sort_by(|a, b| a.path.cmp(&b.path));
        let source_paths = projects.iter().flat_map(|p| p.files.iter().map(|f| f.path.clone()));
        let source_map = SourceMap::from_paths(source_paths);
        Ok(Workspace { root, projects, solution: None, source_map })
    }

    fn from_dir(path: &Path) -> Result<Workspace> { Self::from_dir_with_options(path, WorkspaceLoadOptions::default()) }

    fn from_dir_with_options(path: &Path, opts: WorkspaceLoadOptions) -> Result<Workspace> {
        // Heuristic: prefer a solution in the directory; otherwise, all csproj files directly under dir.
        let mut slns: Vec<PathBuf> = Vec::new();
        let mut csprojs: Vec<PathBuf> = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let p = entry.path();
            if p.is_file() {
                if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
                    let ext = ext.to_ascii_lowercase();
                    if ext == "sln" { slns.push(p.clone()); }
                    if ext == "csproj" { csprojs.push(p); }
                }
            }
        }
        if let Some(sln) = slns.into_iter().next() { return Self::from_sln_with_options(&sln, opts); }
        if let Some(csproj) = csprojs.into_iter().next() { return Self::from_csproj_with_options(&csproj, opts); }
        Err(WorkspaceError::Unsupported(format!("No .sln or .csproj found in {}", path.display())))
    }

    fn follow_project_references(root: &Path, projects: &mut Vec<Project>) {
        let mut seen: HashSet<PathBuf> = projects.iter().map(|p| p.path.clone()).collect();
        let mut queue: VecDeque<PathBuf> = projects
            .iter()
            .flat_map(|p| p.project_references.iter().cloned())
            .collect();

        while let Some(ref_path) = queue.pop_front() {
            // Limit traversal to within root
            if !ref_path.starts_with(root) { continue; }
            if seen.contains(&ref_path) { continue; }
            match CsprojReader::read(&ref_path) {
                Ok(prj) => {
                    seen.insert(prj.path.clone());
                    // enqueue further references
                    for r in prj.project_references.iter() { queue.push_back(r.clone()); }
                    projects.push(prj);
                }
                Err(e) => {
                    // Record a stub with error to keep track
                    let mut prj = Project { name: ref_path.file_stem().and_then(|s| s.to_str()).unwrap_or("project").to_string(), path: ref_path.clone(), ..Default::default() };
                    prj.errors.push(format!("{}", e));
                    seen.insert(ref_path.clone());
                    projects.push(prj);
                }
            }
        }
    }
}
