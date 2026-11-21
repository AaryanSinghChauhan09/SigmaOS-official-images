//! Error types for the official-images component.

/// Errors that can occur rendering image templates.
#[derive(Debug, thiserror::Error)]
pub enum ImageError {
    /// Template rendering failed (missing variable, bad syntax).
    #[error("template error: {0}")]
    Template(String),

    /// I/O error.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
