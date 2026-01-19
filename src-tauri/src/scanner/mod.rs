//! Repository scanner module.
//!
//! Finds git repositories within configured paths.

mod finder;
mod repo_info;

#[cfg(test)]
mod tests;

pub use finder::*;
pub use repo_info::*;

use std::path::Path;
use crate::error::Result;

/// Scan a directory for git repositories.
///
/// # Arguments
/// * `base_path` - The directory to scan
/// * `max_depth` - Maximum directory depth to traverse
///
/// # Returns
/// A list of discovered repositories.
pub fn scan_directory(base_path: &Path, max_depth: u32) -> Result<Vec<DiscoveredRepo>> {
    finder::find_git_repos(base_path, max_depth)
}
