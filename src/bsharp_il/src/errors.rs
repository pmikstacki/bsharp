use thiserror::Error;

#[derive(Debug, Error)]
pub enum IlError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Dotscope error: {0}")]
    Dotscope(#[from] dotscope::Error),
    #[error("Not found: {0}")]
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, IlError>;
