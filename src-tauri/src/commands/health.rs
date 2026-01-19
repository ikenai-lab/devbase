//! Health check commands for verifying app status.

use serde::Serialize;

/// Application health status response.
#[derive(Debug, Serialize)]
pub struct HealthStatus {
    pub status: String,
    pub version: String,
    pub database_connected: bool,
}

/// Check if the application is healthy and responsive.
#[tauri::command]
pub fn health_check() -> HealthStatus {
    tracing::info!("Health check requested");
    
    HealthStatus {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database_connected: true, // Will be dynamic in Phase 1.3
    }
}

/// Get application version information.
#[tauri::command]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
