use thiserror::Error;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, WorkspaceError>;

#[derive(Debug, Error)]
pub enum WorkspaceError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Solution parse error: {0}")]
    SolutionParse(String),

    #[error("Project parse error: {0}")]
    ProjectParse(String),

    #[error("Glob error: {0}")]
    Glob(String),

    #[error("Unsupported file type: {0}")]
    Unsupported(String),

    #[error("Other: {0}")]
    Other(String),
}

impl WorkspaceError {
    pub fn invalid_path(p: &PathBuf) -> Self { Self::InvalidPath(p.display().to_string()) }
}
