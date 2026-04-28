use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServeError {
    #[error("path traversal attempt: {0}")]
    PathTraversal(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, ServeError>;
