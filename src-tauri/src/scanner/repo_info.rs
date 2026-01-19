//! Repository information extraction.
//!
//! Extracts metadata from discovered git repositories.

use std::path::{Path, PathBuf};
use git2::Repository;
use serde::Serialize;

use crate::error::{DevBaseError, Result};

/// Information about a discovered repository.
#[derive(Debug, Clone, Serialize)]
pub struct DiscoveredRepo {
    /// Absolute path to the repository
    pub path: PathBuf,
    /// Repository name (directory name)
    pub name: String,
    /// Remote origin URL (if exists)
    pub remote_url: Option<String>,
    /// Default branch name
    pub default_branch: Option<String>,
    /// Current branch name
    pub current_branch: Option<String>,
}

/// Extract repository information from a path.
pub fn extract_repo_info(path: &Path) -> Result<DiscoveredRepo> {
    let repo = Repository::open(path).map_err(|e| DevBaseError::Scan {
        message: format!("Failed to open repository: {e}"),
    })?;

    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let remote_url = get_remote_url(&repo);
    let default_branch = get_default_branch(&repo);
    let current_branch = get_current_branch(&repo);

    Ok(DiscoveredRepo {
        path: path.to_path_buf(),
        name,
        remote_url,
        default_branch,
        current_branch,
    })
}

/// Get the remote origin URL.
fn get_remote_url(repo: &Repository) -> Option<String> {
    repo.find_remote("origin")
        .ok()
        .and_then(|remote| remote.url().map(String::from))
}

/// Get the default branch (main or master).
fn get_default_branch(repo: &Repository) -> Option<String> {
    // Try to find HEAD reference
    if let Ok(head) = repo.head() {
        if head.is_branch() {
            return head.shorthand().map(String::from);
        }
    }

    // Fallback: check for common default branches
    for branch_name in &["main", "master"] {
        if repo.find_branch(branch_name, git2::BranchType::Local).is_ok() {
            return Some((*branch_name).to_string());
        }
    }

    None
}

/// Get the current branch name.
fn get_current_branch(repo: &Repository) -> Option<String> {
    repo.head()
        .ok()
        .and_then(|head| head.shorthand().map(String::from))
}

/// Extract organization/owner from a remote URL.
pub fn extract_org_from_url(url: &str) -> Option<String> {
    // Handle SSH URLs: git@github.com:owner/repo.git
    if url.starts_with("git@") {
        let parts: Vec<&str> = url.split(':').collect();
        if parts.len() >= 2 {
            let path = parts[1];
            let segments: Vec<&str> = path.split('/').collect();
            if !segments.is_empty() {
                return Some(segments[0].to_string());
            }
        }
    }

    // Handle HTTPS URLs: https://github.com/owner/repo.git
    if url.starts_with("http") {
        if let Ok(parsed) = url::Url::parse(url) {
            let segments: Vec<&str> = parsed.path().split('/').filter(|s| !s.is_empty()).collect();
            if !segments.is_empty() {
                return Some(segments[0].to_string());
            }
        }
    }

    None
}

/// Extract the host from a remote URL.
pub fn extract_host_from_url(url: &str) -> Option<String> {
    // Handle SSH URLs: git@github.com:owner/repo.git
    if url.starts_with("git@") {
        let parts: Vec<&str> = url.split('@').collect();
        if parts.len() >= 2 {
            let host_part = parts[1].split(':').next()?;
            return Some(host_part.to_string());
        }
    }

    // Handle HTTPS URLs
    if url.starts_with("http") {
        if let Ok(parsed) = url::Url::parse(url) {
            return parsed.host_str().map(String::from);
        }
    }

    None
}
