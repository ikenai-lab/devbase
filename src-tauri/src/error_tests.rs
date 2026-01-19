//! Tests for error handling module.

#[cfg(test)]
mod tests {
    use crate::error::{DevBaseError, IpcError};

    #[test]
    fn test_database_error_display() {
        let err = DevBaseError::Database(rusqlite::Error::QueryReturnedNoRows);
        assert!(err.to_string().contains("Database error"));
    }

    #[test]
    fn test_config_error_display() {
        let err = DevBaseError::Config {
            message: "Invalid path".to_string(),
        };
        assert_eq!(err.to_string(), "Configuration error: Invalid path");
    }

    #[test]
    fn test_scan_error_display() {
        let err = DevBaseError::Scan {
            message: "Permission denied".to_string(),
        };
        assert_eq!(err.to_string(), "Scan error: Permission denied");
    }

    #[test]
    fn test_internal_error_display() {
        let err = DevBaseError::Internal {
            message: "Unexpected state".to_string(),
        };
        assert_eq!(err.to_string(), "Internal error: Unexpected state");
    }

    #[test]
    fn test_ipc_error_from_devbase_error() {
        let err = DevBaseError::Config {
            message: "Bad config".to_string(),
        };
        let ipc_err: IpcError = err.into();
        
        assert_eq!(ipc_err.code, "CONFIG_ERROR");
        assert!(ipc_err.message.contains("Bad config"));
    }

    #[test]
    fn test_error_serialization() {
        let err = DevBaseError::Internal {
            message: "Test error".to_string(),
        };
        
        // Should serialize without panicking
        let result = serde_json::to_string(&err);
        assert!(result.is_ok());
    }
}
