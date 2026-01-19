//! Error types for DevBase application.
//!
//! All errors use `thiserror` for ergonomic error handling.

use serde::Serialize;
use thiserror::Error;

/// Main error type for DevBase operations.
#[derive(Debug, Error)]
pub enum DevBaseError {
    /// Database-related errors.
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    /// File system errors.
    #[error("File system error: {0}")]
    FileSystem(#[from] std::io::Error),

    /// Git errors.
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    /// Configuration errors.
    #[error("Configuration error: {message}")]
    Config { message: String },

    /// Repository scanning errors.
    #[error("Scan error: {message}")]
    Scan { message: String },

    /// General internal errors.
    #[error("Internal error: {message}")]
    Internal { message: String },
}

/// Serializable error for Tauri IPC responses.
#[derive(Debug, Serialize)]
pub struct IpcError {
    pub code: String,
    pub message: String,
}

impl From<DevBaseError> for IpcError {
    fn from(err: DevBaseError) -> Self {
        let code = match &err {
            DevBaseError::Database(_) => "DATABASE_ERROR",
            DevBaseError::FileSystem(_) => "FILESYSTEM_ERROR",
            DevBaseError::Git(_) => "GIT_ERROR",
            DevBaseError::Config { .. } => "CONFIG_ERROR",
            DevBaseError::Scan { .. } => "SCAN_ERROR",
            DevBaseError::Internal { .. } => "INTERNAL_ERROR",
        };

        Self {
            code: code.to_string(),
            message: err.to_string(),
        }
    }
}

// Implement Serialize for DevBaseError to work with Tauri commands
impl Serialize for DevBaseError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        IpcError::from(DevBaseError::Internal {
            message: self.to_string(),
        })
        .serialize(serializer)
    }
}

/// Result type alias for DevBase operations.
pub type Result<T> = std::result::Result<T, DevBaseError>;

#[cfg(test)]
#[path = "error_tests.rs"]
mod tests;
