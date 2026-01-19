//! Git repository finder.
//!
//! Recursively searches directories for .git folders.

use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

use crate::error::{DevBaseError, Result};
use super::repo_info::{DiscoveredRepo, extract_repo_info};

/// Check if a directory entry is a `.git` directory.
fn is_git_dir(entry: &DirEntry) -> bool {
    entry.file_type().is_dir() && entry.file_name() == ".git"
}

/// Check if a path should be skipped (common non-repo directories).
/// Note: We do NOT skip .git - we need to find these directories.
/// The walker won't descend into them after we find them.
fn should_skip(entry: &DirEntry) -> bool {
    let name = entry.file_name().to_string_lossy();
    
    // Skip common non-repo directories for performance
    // Do NOT skip .git - we need to find it to identify repos
    matches!(
        name.as_ref(),
        "node_modules" | "target" | "vendor" | ".cargo" | 
        "__pycache__" | ".venv" | "venv" | ".tox" | "dist" | "build"
    )
}

/// Find all git repositories within a directory.
///
/// # Arguments
/// * `base_path` - The root directory to scan
/// * `max_depth` - Maximum directory depth (0 = unlimited)
///
/// # Returns
/// A vector of discovered repository information.
pub fn find_git_repos(base_path: &Path, max_depth: u32) -> Result<Vec<DiscoveredRepo>> {
    tracing::info!(?base_path, max_depth, "Starting repository scan");

    if !base_path.exists() {
        return Err(DevBaseError::Scan {
            message: format!("Path does not exist: {}", base_path.display()),
        });
    }

    if !base_path.is_dir() {
        return Err(DevBaseError::Scan {
            message: format!("Path is not a directory: {}", base_path.display()),
        });
    }

    let mut repos = Vec::new();
    let mut seen_repos: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();

    let walker = if max_depth > 0 {
        WalkDir::new(base_path).max_depth(max_depth as usize)
    } else {
        WalkDir::new(base_path)
    };

    for entry in walker
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| !should_skip(e))
        .filter_map(|e| e.ok())
    {
        if is_git_dir(&entry) {
            // Get the parent directory (the actual repo, not .git)
            if let Some(repo_path) = entry.path().parent() {
                let repo_path = repo_path.to_path_buf();
                
                // Skip if we've already seen this repo (handles symlinks)
                if seen_repos.contains(&repo_path) {
                    continue;
                }
                seen_repos.insert(repo_path.clone());

                // Skip if this is inside another repo (submodule)
                let is_submodule = seen_repos.iter().any(|existing| {
                    repo_path.starts_with(existing) && repo_path != *existing
                });
                
                if is_submodule {
                    tracing::debug!(?repo_path, "Skipping submodule");
                    continue;
                }

                match extract_repo_info(&repo_path) {
                    Ok(info) => {
                        tracing::debug!(?repo_path, "Found repository");
                        repos.push(info);
                    }
                    Err(e) => {
                        tracing::warn!(?repo_path, ?e, "Failed to extract repo info");
                    }
                }
            }
        }
    }

    tracing::info!(count = repos.len(), "Scan complete");
    Ok(repos)
}

/// Check if a path is a git repository.
pub fn is_git_repo(path: &Path) -> bool {
    path.join(".git").is_dir()
}
