//! Tests for commands module.

#[cfg(test)]
mod tests {
    use crate::commands::{health_check, get_version};

    #[test]
    fn test_health_check_returns_ok_status() {
        let status = health_check();
        assert_eq!(status.status, "ok");
    }

    #[test]
    fn test_health_check_returns_version() {
        let status = health_check();
        assert!(!status.version.is_empty());
        assert_eq!(status.version, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn test_health_check_database_connected() {
        let status = health_check();
        // For now, always returns true (will be dynamic later)
        assert!(status.database_connected);
    }

    #[test]
    fn test_get_version_returns_cargo_version() {
        let version = get_version();
        assert_eq!(version, env!("CARGO_PKG_VERSION"));
        assert_eq!(version, "0.1.0");
    }

    #[test]
    fn test_health_status_serialization() {
        let status = health_check();
        let json = serde_json::to_string(&status);
        assert!(json.is_ok());
        
        let json_str = json.unwrap();
        assert!(json_str.contains("\"status\":\"ok\""));
        assert!(json_str.contains("\"database_connected\":true"));
    }
}
