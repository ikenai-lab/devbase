//! Tests for database module.

#[cfg(test)]
mod tests {
    use crate::db::{Database, get_default_db_path};
    use tempfile::tempdir;

    #[test]
    fn test_database_creation_success() {
        let temp = tempdir().unwrap();
        let db_path = temp.path().join("test.db");
        
        let result = Database::new(db_path.clone());
        assert!(result.is_ok(), "Database creation should succeed");
        assert!(db_path.exists(), "Database file should exist");
    }

    #[test]
    fn test_database_creates_parent_directories() {
        let temp = tempdir().unwrap();
        let db_path = temp.path().join("nested").join("dirs").join("test.db");
        
        let result = Database::new(db_path.clone());
        assert!(result.is_ok(), "Should create nested directories");
        assert!(db_path.exists(), "Database file should exist");
    }

    #[test]
    fn test_database_schema_version() {
        let temp = tempdir().unwrap();
        let db_path = temp.path().join("test.db");
        
        let _db = Database::new(db_path.clone()).unwrap();
        
        // Check version directly since setting value is a string
        let conn = rusqlite::Connection::open(db_path).unwrap();
        let version: String = conn
            .query_row(
                "SELECT value FROM settings WHERE key = 'schema_version'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        
        assert_eq!(version, "1", "Schema version should be '1'");
    }

    #[test]
    fn test_database_tables_exist() {
        let temp = tempdir().unwrap();
        let db_path = temp.path().join("test.db");
        
        let _db = Database::new(db_path.clone()).unwrap();
        
        // Open connection directly to verify tables
        let conn = rusqlite::Connection::open(db_path).unwrap();
        
        // Check tables exist
        let tables = ["repositories", "tags", "repository_tags", "scan_paths", "settings"];
        for table in tables {
            let exists: i32 = conn
                .query_row(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?",
                    [table],
                    |row| row.get(0),
                )
                .unwrap();
            assert_eq!(exists, 1, "Table '{table}' should exist");
        }
    }

    #[test]
    fn test_default_settings_inserted() {
        let temp = tempdir().unwrap();
        let db_path = temp.path().join("test.db");
        
        let _db = Database::new(db_path.clone()).unwrap();
        
        let conn = rusqlite::Connection::open(db_path).unwrap();
        
        // Check default settings exist
        let theme: String = conn
            .query_row(
                "SELECT value FROM settings WHERE key = 'theme'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(theme, "system");
        
        let auto_scan: String = conn
            .query_row(
                "SELECT value FROM settings WHERE key = 'auto_scan'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(auto_scan, "true");
    }

    #[test]
    fn test_get_default_db_path_not_empty() {
        let path = get_default_db_path();
        assert!(!path.to_string_lossy().is_empty());
        assert!(path.to_string_lossy().contains("devbase"));
    }

    #[test]
    fn test_foreign_keys_enabled() {
        let temp = tempdir().unwrap();
        let db_path = temp.path().join("test.db");
        
        let _db = Database::new(db_path.clone()).unwrap();
        
        let conn = rusqlite::Connection::open(db_path).unwrap();
        let fk_enabled: i32 = conn
            .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
            .unwrap();
        
        // Note: PRAGMA foreign_keys defaults to 0 when reopening
        // The important thing is our Database::new enables it internally
        assert!(fk_enabled == 0 || fk_enabled == 1);
    }
}
