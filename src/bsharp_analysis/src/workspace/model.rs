use crate::workspace::source_map::SourceMap;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    CSharp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProjectFileKind {
    Source,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectFile {
    pub path: PathBuf,
    pub kind: ProjectFileKind,
    pub language: Option<Language>,
}

impl ProjectFile {
    pub fn new_source(path: PathBuf, language: Language) -> Self {
        Self {
            path,
            kind: ProjectFileKind::Source,
            language: Some(language),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRef {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Solution {
    pub path: PathBuf,
    pub projects: Vec<ProjectRef>,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
    pub files: Vec<ProjectFile>,
    pub project_references: Vec<PathBuf>,
    pub errors: Vec<String>,
}

impl Project {
    pub fn add_file(&mut self, file: ProjectFile) {
        self.files.push(file);
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Workspace {
    pub root: PathBuf,
    pub projects: Vec<Project>,
    pub solution: Option<Solution>,
    #[serde(skip)]
    pub source_map: SourceMap,
}

impl Workspace {
    pub fn all_source_files(&self) -> Vec<&Path> {
        let mut v = Vec::new();
        for p in &self.projects {
            for f in &p.files {
                if matches!(f.kind, ProjectFileKind::Source) {
                    v.push(f.path.as_path());
                }
            }
        }
        v
    }
}
