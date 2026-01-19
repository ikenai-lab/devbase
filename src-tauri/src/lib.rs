//! DevBase - Local Repository Viewer & Manager
//!
//! This is the core library for the DevBase application.
//! It provides Git repository management, scanning, and visualization.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

pub mod commands;
pub mod db;
pub mod error;

use std::sync::Arc;
use tauri::Manager;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::db::Database;

/// Application state managed by Tauri.
pub struct AppState {
    pub db: Arc<Database>,
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

    // Initialize database
    let db_path = db::get_default_db_path();
    let db = match Database::new(db_path) {
        Ok(db) => Arc::new(db),
        Err(e) => {
            tracing::error!(?e, "Failed to initialize database");
            // For now, panic on database init failure
            // In production, we'd show a user-friendly error dialog
            panic!("Database initialization failed: {e}");
        }
    };

    let app_state = AppState { db };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::health_check,
            commands::get_version,
        ])
        .setup(|app| {
            let window = app.get_webview_window("main");
            if let Some(w) = window {
                tracing::info!("Main window ready");
                // Set window title
                let _ = w.set_title("DevBase - Code Mission Control");
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .unwrap_or_else(|e| {
            tracing::error!(?e, "Error running Tauri application");
        });
}

