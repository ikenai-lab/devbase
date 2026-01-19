//! DevBase - Local Repository Viewer & Manager
//!
//! This is the core library for the DevBase application.
//! It provides Git repository management, scanning, and visualization.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

pub mod db;
pub mod error;
pub mod git;
pub mod scanner;

use std::sync::Arc;
use tauri::Manager;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::db::Database;

/// Application state managed by Tauri.
pub struct AppState {
    pub db: Arc<Database>,
}

// ============= Commands defined inline for Tauri macro compatibility =============

mod commands {
    use std::path::PathBuf;
    use serde::{Deserialize, Serialize};
    use tauri::State;

    use crate::error::{DevBaseError, Result};
    use crate::git::{self, RepoHealth, RepoStatus};
    use crate::scanner::{self, DiscoveredRepo};
    use crate::AppState;

    // ========== Health Commands ==========

    #[derive(Debug, Serialize)]
    pub struct HealthStatus {
        pub status: String,
        pub version: String,
        pub database_connected: bool,
    }

    #[tauri::command]
    pub fn health_check() -> HealthStatus {
        tracing::info!("Health check requested");
        HealthStatus {
            status: "ok".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            database_connected: true,
        }
    }

    #[tauri::command]
    pub fn get_version() -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    // ========== Scan Commands ==========

    #[tauri::command]
    pub async fn start_scan(state: State<'_, AppState>) -> Result<Vec<DiscoveredRepo>> {
        tracing::info!("Starting repository scan");
        
        let scan_paths = state.db.get_scan_paths()?;
        tracing::info!("Found {} scan paths in configuration", scan_paths.len());

        let mut all_repos = Vec::new();

        for scan_path in scan_paths {
            tracing::info!(
                path = ?scan_path.path, 
                enabled = scan_path.enabled, 
                depth = scan_path.max_depth,
                "Processing scan path"
            );

            if !scan_path.enabled {
                tracing::info!(path = ?scan_path.path, "Skipping disabled path");
                continue;
            }

            match scanner::scan_directory(&scan_path.path, scan_path.max_depth) {
                Ok(repos) => {
                    tracing::info!(path = ?scan_path.path, count = repos.len(), "Scanned path");
                    all_repos.extend(repos);
                }
                Err(e) => {
                    tracing::warn!(path = ?scan_path.path, ?e, "Failed to scan path");
                }
            }
        }

        for repo in &all_repos {
            if let Err(e) = state.db.upsert_repository(repo) {
                tracing::warn!(?e, "Failed to save repository");
            }
        }

        Ok(all_repos)
    }

    // Helper to expand tilde in paths
    fn expand_path(path: &str) -> PathBuf {
        if path.starts_with("~") {
            if let Some(home) = dirs::home_dir() {
                if path == "~" {
                    return home;
                }
                if let Some(rest) = path.strip_prefix("~/") {
                    return home.join(rest);
                }
                // Handle case where path is just "~" or "~/"
                if path == "~/" {
                    return home;
                }
            }
        }
        PathBuf::from(path)
    }

    #[tauri::command]
    pub fn scan_path(path: String, max_depth: Option<u32>) -> Result<Vec<DiscoveredRepo>> {
        let path_buf = expand_path(&path);
        let depth = max_depth.unwrap_or(5);
        scanner::scan_directory(&path_buf, depth)
    }

    // ========== Repository Commands ==========

    #[derive(Debug, Clone, Serialize)]
    pub struct RepoInfo {
        pub id: i64,
        pub path: String,
        pub name: String,
        pub remote_url: Option<String>,
        pub default_branch: Option<String>,
        pub current_branch: Option<String>,
        pub health: RepoHealth,
        pub status: RepoStatus,
        pub tags: Vec<String>,
    }

    #[tauri::command]
    pub async fn get_repositories(state: State<'_, AppState>) -> Result<Vec<RepoInfo>> {
        let repos = state.db.get_all_repositories()?;
        let mut result = Vec::with_capacity(repos.len());
        
        for repo in repos {
            let health = git::get_repo_health(&PathBuf::from(&repo.path)).unwrap_or_default();
            let status = health.status();
            let tags = state.db.get_repo_tags(repo.id)?;
            
            result.push(RepoInfo {
                id: repo.id,
                path: repo.path,
                name: repo.name,
                remote_url: repo.remote_url,
                default_branch: repo.default_branch,
                current_branch: health.current_branch.clone(),
                health,
                status,
                tags,
            });
        }
        
        Ok(result)
    }

    #[tauri::command]
    pub fn get_repo_health(path: String) -> Result<RepoHealth> {
        let path_utils = expand_path(&path);
        git::get_repo_health(&path_utils)
    }

    #[tauri::command]
    pub async fn refresh_repo(state: State<'_, AppState>, repo_id: i64) -> Result<RepoInfo> {
        let repo = state.db.get_repository(repo_id)?;
        let health = git::get_repo_health(&PathBuf::from(&repo.path)).unwrap_or_default();
        let status = health.status();
        let tags = state.db.get_repo_tags(repo.id)?;
        
        Ok(RepoInfo {
            id: repo.id,
            path: repo.path,
            name: repo.name,
            remote_url: repo.remote_url,
            default_branch: repo.default_branch,
            current_branch: health.current_branch.clone(),
            health,
            status,
            tags,
        })
    }

    // ========== Settings Commands ==========

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ScanPath {
        pub id: i64,
        pub path: String,
        pub enabled: bool,
        pub max_depth: u32,
    }

    #[tauri::command]
    pub async fn get_scan_paths(state: State<'_, AppState>) -> Result<Vec<ScanPath>> {
        let paths = state.db.get_scan_paths()?;
        Ok(paths.into_iter().map(|p| ScanPath {
            id: p.id,
            path: p.path.to_string_lossy().to_string(),
            enabled: p.enabled,
            max_depth: p.max_depth,
        }).collect())
    }

    #[tauri::command]
    pub async fn add_scan_path(
        state: State<'_, AppState>,
        path: String,
        max_depth: Option<u32>,
    ) -> Result<ScanPath> {
        let path_buf = expand_path(&path);
        
        if !path_buf.exists() {
            return Err(DevBaseError::Config {
                message: format!("Path does not exist: {path}"),
            });
        }
        
        if !path_buf.is_dir() {
            return Err(DevBaseError::Config {
                message: format!("Path is not a directory: {path}"),
            });
        }

        let depth = max_depth.unwrap_or(5);
        let id = state.db.add_scan_path(&path_buf, depth)?;
        
        Ok(ScanPath { id, path, enabled: true, max_depth: depth })
    }

    #[tauri::command]
    pub async fn remove_scan_path(state: State<'_, AppState>, id: i64) -> Result<()> {
        state.db.remove_scan_path(id)
    }

    #[tauri::command]
    pub async fn update_scan_path(
        state: State<'_, AppState>,
        id: i64,
        enabled: Option<bool>,
        max_depth: Option<u32>,
    ) -> Result<()> {
        state.db.update_scan_path(id, enabled, max_depth)
    }

    #[tauri::command]
    pub async fn get_setting(state: State<'_, AppState>, key: String) -> Result<Option<String>> {
        state.db.get_setting(&key)
    }

    #[tauri::command]
    pub async fn set_setting(state: State<'_, AppState>, key: String, value: String) -> Result<()> {
        state.db.set_setting(&key, &value)
    }

    // ========== Tag Commands ==========

    #[tauri::command]
    pub async fn get_tags(state: State<'_, AppState>) -> Result<Vec<crate::db::Tag>> {
        state.db.get_all_tags()
    }

    #[tauri::command]
    pub async fn create_tag(
        state: State<'_, AppState>,
        name: String,
        color: Option<String>,
    ) -> Result<crate::db::Tag> {
        let color = color.unwrap_or_else(|| "#808080".to_string());
        let id = state.db.create_tag(&name, &color)?;
        Ok(crate::db::Tag { id, name, color })
    }

    #[tauri::command]
    pub async fn delete_tag(state: State<'_, AppState>, id: i64) -> Result<()> {
        state.db.delete_tag(id)
    }

    #[tauri::command]
    pub async fn assign_tag(state: State<'_, AppState>, repo_id: i64, tag_id: i64) -> Result<()> {
        state.db.assign_tag(repo_id, tag_id)
    }

    #[tauri::command]
    pub fn remove_tag(state: State<'_, AppState>, repo_id: i64, tag_id: i64) -> Result<()> {
        state.db.remove_tag(repo_id, tag_id)
    }

    // ========== History Commands ==========

    #[tauri::command]
    pub fn get_commit_log(path: String, limit: Option<usize>) -> Result<Vec<crate::git::history::CommitLogEntry>> {
        let path_buf = expand_path(&path);
        let limit = limit.unwrap_or(100);
        crate::git::history::get_repo_history(&path_buf, limit)
    }
}

/// Initialize the tracing subscriber for logging.
fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "devbase=debug,tauri=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Tracing initialized");
}

/// Main entry point for the Tauri application.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_tracing();

    tracing::info!("Starting DevBase application");

    let db_path = db::get_default_db_path();
    let db = match Database::new(db_path) {
        Ok(db) => Arc::new(db),
        Err(e) => {
            tracing::error!(?e, "Failed to initialize database");
            panic!("Database initialization failed: {e}");
        }
    };

    let app_state = AppState { db };

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            // Health
            commands::health_check,
            commands::get_version,
            // Scan
            commands::start_scan,
            commands::scan_path,
            // Repositories
            commands::get_repositories,
            commands::get_repo_health,
            commands::refresh_repo,
            // Settings
            commands::get_scan_paths,
            commands::add_scan_path,
            commands::remove_scan_path,
            commands::update_scan_path,
            commands::get_setting,
            commands::set_setting,
            // Tags
            commands::get_tags,
            commands::create_tag,
            commands::delete_tag,
            commands::assign_tag,
            commands::remove_tag,
            // History
            commands::get_commit_log,
        ])
        .setup(|app| {
            let window = app.get_webview_window("main");
            if let Some(w) = window {
                tracing::info!("Main window ready");
                let _ = w.set_title("DevBase - Code Mission Control");
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .unwrap_or_else(|e| {
            tracing::error!(?e, "Error running Tauri application");
        });
}
