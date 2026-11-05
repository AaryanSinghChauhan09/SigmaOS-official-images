//! Official container base image definitions
//!
//! SigmaOS component inspired by Arch Linux's `official-images` repository.
//! Designed for component isolation, loose coupling, and secure defaults.

use std::fmt;

/// Errors produced by this component. Fail-fast: errors are returned, not panicked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sigmaos_official_imagesError {
    /// The requested operation is not valid in the current state.
    InvalidState,
    /// Input failed validation.
    InvalidInput(String),
}

impl fmt::Display for Sigmaos_official_imagesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidState => write!(f, "invalid state"),
            Self::InvalidInput(msg) => write!(f, "invalid input: {msg}"),
        }
    }
}

impl std::error::Error for Sigmaos_official_imagesError {}

/// Core component type. Holds configuration; performs no I/O by default.
#[derive(Debug, Clone, Default)]
pub struct Sigmaos_official_images {
    name: String,
}

impl Sigmaos_official_images {
    /// Create a new component instance. Fails fast on empty names.
    pub fn new(name: impl Into<String>) -> Result<Self, Sigmaos_official_imagesError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(Sigmaos_official_imagesError::InvalidInput("name must not be empty".into()));
        }
        Ok(Self { name })
    }

    /// Component identifier.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Perform the component's primary action. Returns a deterministic result.
    pub fn run(&self) -> Result<String, Sigmaos_official_imagesError> {
        if self.name.is_empty() {
            return Err(Sigmaos_official_imagesError::InvalidState);
        }
        Ok(format!("{}: ok", self.name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_empty_name() {
        // Arrange / Act / Assert
        assert!(matches!(Sigmaos_official_images::new("  "), Err(Sigmaos_official_imagesError::InvalidInput(_))));
    }

    #[test]
    fn run_returns_deterministic_result() {
        let c = Sigmaos_official_images::new("test").unwrap();
        assert_eq!(c.run().unwrap(), "test: ok");
    }

    #[test]
    fn name_is_accessible() {
        let c = Sigmaos_official_images::new("abc").unwrap();
        assert_eq!(c.name(), "abc");
    }
}
