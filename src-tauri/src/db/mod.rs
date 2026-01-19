//! Database module for SQLite operations.
//!
//! Handles database connection, schema initialization, and migrations.

mod schema;

#[cfg(test)]
mod tests;

use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;

use crate::error::{DevBaseError, Result};

/// Database manager holding the SQLite connection.
pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    /// Create a new database connection.
    ///
    /// # Errors
    ///
    /// Returns an error if the database file cannot be created or if
    /// schema initialization fails.
    pub fn new(db_path: PathBuf) -> Result<Self> {
        tracing::info!(?db_path, "Initializing database");

        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(&db_path)?;
        
        // Enable foreign keys
        conn.execute("PRAGMA foreign_keys = ON", [])?;
        
        let db = Self {
            conn: Mutex::new(conn),
        };

        // Initialize schema
        db.init_schema()?;

        tracing::info!("Database initialized successfully");
        Ok(db)
    }

    /// Initialize the database schema.
    fn init_schema(&self) -> Result<()> {
        let conn = self.conn.lock().map_err(|e| DevBaseError::Internal {
            message: format!("Failed to acquire database lock: {e}"),
        })?;

        conn.execute_batch(schema::INIT_SCHEMA)?;
        
        tracing::debug!("Database schema initialized");
        Ok(())
    }

    /// Get the database version.
    pub fn get_version(&self) -> Result<i32> {
        let conn = self.conn.lock().map_err(|e| DevBaseError::Internal {
            message: format!("Failed to acquire database lock: {e}"),
        })?;

        let version: i32 = conn.query_row(
            "SELECT value FROM settings WHERE key = 'schema_version'",
            [],
            |row| row.get(0),
        ).unwrap_or(0);

        Ok(version)
    }
}

/// Get the default database path in the app data directory.
pub fn get_default_db_path() -> PathBuf {
    let data_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("devbase");
    
    data_dir.join("devbase.db")
}
