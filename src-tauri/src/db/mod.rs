//! Database module for SQLite operations.
//!
//! Handles database connection, schema initialization, and CRUD operations.

mod schema;

#[cfg(test)]
mod tests;

use rusqlite::{params, Connection};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use crate::error::{DevBaseError, Result};
use crate::scanner::DiscoveredRepo;

/// Stored repository record.
#[derive(Debug, Clone)]
pub struct StoredRepo {
    pub id: i64,
    pub path: String,
    pub name: String,
    pub remote_url: Option<String>,
    pub default_branch: Option<String>,
}

/// Scan path configuration.
#[derive(Debug, Clone)]
pub struct ScanPathConfig {
    pub id: i64,
    pub path: PathBuf,
    pub enabled: bool,
    pub max_depth: u32,
}

/// Tag information (used internally in db).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub color: String,
}

/// Database manager holding the SQLite connection.
pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    /// Create a new database connection.
    pub fn new(db_path: PathBuf) -> Result<Self> {
        tracing::info!(?db_path, "Initializing database");

        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(&db_path)?;
        conn.execute("PRAGMA foreign_keys = ON", [])?;
        
        let db = Self {
            conn: Mutex::new(conn),
        };

        db.init_schema()?;

        tracing::info!("Database initialized successfully");
        Ok(db)
    }

    fn init_schema(&self) -> Result<()> {
        let conn = self.lock()?;
        conn.execute_batch(schema::INIT_SCHEMA)?;
        tracing::debug!("Database schema initialized");
        Ok(())
    }

    fn lock(&self) -> Result<std::sync::MutexGuard<'_, Connection>> {
        self.conn.lock().map_err(|e| DevBaseError::Internal {
            message: format!("Failed to acquire database lock: {e}"),
        })
    }

    // ========== Repository Methods ==========

    pub fn upsert_repository(&self, repo: &DiscoveredRepo) -> Result<i64> {
        let conn = self.lock()?;
        
        conn.execute(
            "INSERT INTO repositories (path, name, remote_url, default_branch, last_scanned_at)
             VALUES (?1, ?2, ?3, ?4, CURRENT_TIMESTAMP)
             ON CONFLICT(path) DO UPDATE SET
                name = excluded.name,
                remote_url = excluded.remote_url,
                default_branch = excluded.default_branch,
                last_scanned_at = CURRENT_TIMESTAMP",
            params![
                repo.path.to_string_lossy().to_string(),
                repo.name,
                repo.remote_url,
                repo.default_branch,
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    pub fn get_all_repositories(&self) -> Result<Vec<StoredRepo>> {
        let conn = self.lock()?;
        let mut stmt = conn.prepare(
            "SELECT id, path, name, remote_url, default_branch FROM repositories ORDER BY name"
        )?;

        let repos = stmt.query_map([], |row| {
            Ok(StoredRepo {
                id: row.get(0)?,
                path: row.get(1)?,
                name: row.get(2)?,
                remote_url: row.get(3)?,
                default_branch: row.get(4)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

        Ok(repos)
    }

    pub fn get_repository(&self, id: i64) -> Result<StoredRepo> {
        let conn = self.lock()?;
        
        conn.query_row(
            "SELECT id, path, name, remote_url, default_branch FROM repositories WHERE id = ?1",
            [id],
            |row| {
                Ok(StoredRepo {
                    id: row.get(0)?,
                    path: row.get(1)?,
                    name: row.get(2)?,
                    remote_url: row.get(3)?,
                    default_branch: row.get(4)?,
                })
            },
        ).map_err(DevBaseError::Database)
    }

    // ========== Scan Path Methods ==========

    pub fn get_scan_paths(&self) -> Result<Vec<ScanPathConfig>> {
        let conn = self.lock()?;
        let mut stmt = conn.prepare(
            "SELECT id, path, enabled, max_depth FROM scan_paths ORDER BY path"
        )?;

        let paths = stmt.query_map([], |row| {
            Ok(ScanPathConfig {
                id: row.get(0)?,
                path: PathBuf::from(row.get::<_, String>(1)?),
                enabled: row.get::<_, i32>(2)? != 0,
                max_depth: row.get::<_, i32>(3)? as u32,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

        Ok(paths)
    }

    pub fn add_scan_path(&self, path: &Path, max_depth: u32) -> Result<i64> {
        let conn = self.lock()?;
        
        conn.execute(
            "INSERT INTO scan_paths (path, max_depth) VALUES (?1, ?2)",
            params![path.to_string_lossy().to_string(), max_depth as i32],
        )?;

        Ok(conn.last_insert_rowid())
    }

    pub fn remove_scan_path(&self, id: i64) -> Result<()> {
        let conn = self.lock()?;
        conn.execute("DELETE FROM scan_paths WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn update_scan_path(&self, id: i64, enabled: Option<bool>, max_depth: Option<u32>) -> Result<()> {
        let conn = self.lock()?;
        
        if let Some(e) = enabled {
            conn.execute("UPDATE scan_paths SET enabled = ?1 WHERE id = ?2", params![e as i32, id])?;
        }
        if let Some(d) = max_depth {
            conn.execute("UPDATE scan_paths SET max_depth = ?1 WHERE id = ?2", params![d as i32, id])?;
        }
        
        Ok(())
    }

    // ========== Tag Methods ==========

    pub fn get_all_tags(&self) -> Result<Vec<Tag>> {
        let conn = self.lock()?;
        let mut stmt = conn.prepare("SELECT id, name, color FROM tags ORDER BY name")?;

        let tags = stmt.query_map([], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

        Ok(tags)
    }

    pub fn create_tag(&self, name: &str, color: &str) -> Result<i64> {
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO tags (name, color) VALUES (?1, ?2)",
            params![name, color],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn delete_tag(&self, id: i64) -> Result<()> {
        let conn = self.lock()?;
        conn.execute("DELETE FROM tags WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn assign_tag(&self, repo_id: i64, tag_id: i64) -> Result<()> {
        let conn = self.lock()?;
        conn.execute(
            "INSERT OR IGNORE INTO repository_tags (repo_id, tag_id) VALUES (?1, ?2)",
            params![repo_id, tag_id],
        )?;
        Ok(())
    }

    pub fn remove_tag(&self, repo_id: i64, tag_id: i64) -> Result<()> {
        let conn = self.lock()?;
        conn.execute(
            "DELETE FROM repository_tags WHERE repo_id = ?1 AND tag_id = ?2",
            params![repo_id, tag_id],
        )?;
        Ok(())
    }

    pub fn get_repo_tags(&self, repo_id: i64) -> Result<Vec<String>> {
        let conn = self.lock()?;
        let mut stmt = conn.prepare(
            "SELECT t.name FROM tags t 
             JOIN repository_tags rt ON t.id = rt.tag_id 
             WHERE rt.repo_id = ?1 ORDER BY t.name"
        )?;

        let tags: Vec<String> = stmt.query_map([repo_id], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();

        Ok(tags)
    }

    // ========== Settings Methods ==========

    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let conn = self.lock()?;
        let result = conn.query_row(
            "SELECT value FROM settings WHERE key = ?1",
            [key],
            |row| row.get(0),
        );

        match result {
            Ok(v) => Ok(Some(v)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(DevBaseError::Database(e)),
        }
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = CURRENT_TIMESTAMP",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn get_version(&self) -> Result<i32> {
        let conn = self.lock()?;
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
