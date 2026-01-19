//! Git repository status checking.

use std::path::Path;
use git2::{Repository, StatusOptions};
use serde::Serialize;

use crate::error::{DevBaseError, Result};

/// Repository health/status information.
#[derive(Debug, Clone, Serialize, Default)]
pub struct RepoHealth {
    /// Repository has uncommitted changes
    pub is_dirty: bool,
    /// Number of uncommitted files
    pub uncommitted_count: u32,
    /// Number of staged files
    pub staged_count: u32,
    /// Number of commits ahead of remote
    pub commits_ahead: u32,
    /// Number of commits behind remote
    pub commits_behind: u32,
    /// Number of stashed changes
    pub stash_count: u32,
    /// Current branch name
    pub current_branch: Option<String>,
    /// Whether the repo is on a detached HEAD
    pub is_detached: bool,
}

/// Get the health status of a repository.
pub fn get_repo_health(path: &Path) -> Result<RepoHealth> {
    let mut repo = Repository::open(path).map_err(|e| DevBaseError::Scan {
        message: format!("Failed to open repository: {e}"),
    })?;

    let mut health = RepoHealth::default();

    // Get current branch
    if let Ok(head) = repo.head() {
        health.is_detached = !head.is_branch();
        health.current_branch = head.shorthand().map(String::from);
    }

    // Get status
    let mut opts = StatusOptions::new();
    opts.include_untracked(true)
        .recurse_untracked_dirs(true);

    if let Ok(statuses) = repo.statuses(Some(&mut opts)) {
        for status in statuses.iter() {
            let s = status.status();
            
            if s.is_index_new() || s.is_index_modified() || s.is_index_deleted() ||
               s.is_index_renamed() || s.is_index_typechange() {
                health.staged_count += 1;
            }
            
            if s.is_wt_new() || s.is_wt_modified() || s.is_wt_deleted() ||
               s.is_wt_renamed() || s.is_wt_typechange() {
                health.uncommitted_count += 1;
            }
        }
    }

    health.is_dirty = health.uncommitted_count > 0 || health.staged_count > 0;

    // Get stash count
    let mut stash_count = 0u32;
    let _ = repo.stash_foreach(|_, _, _| {
        stash_count += 1;
        true
    });
    health.stash_count = stash_count;

    // Get ahead/behind count
    if let Some(branch_name) = &health.current_branch {
        if let Ok((ahead, behind)) = get_ahead_behind(&repo, branch_name) {
            health.commits_ahead = ahead;
            health.commits_behind = behind;
        }
    }

    Ok(health)
}

/// Get the number of commits ahead and behind the upstream.
fn get_ahead_behind(repo: &Repository, branch_name: &str) -> Result<(u32, u32)> {
    let branch = repo.find_branch(branch_name, git2::BranchType::Local)
        .map_err(|e| DevBaseError::Scan {
            message: format!("Failed to find branch: {e}"),
        })?;

    let upstream = match branch.upstream() {
        Ok(u) => u,
        Err(_) => return Ok((0, 0)), // No upstream configured
    };

    let local_oid = branch.get().target().ok_or_else(|| DevBaseError::Scan {
        message: "No local target".to_string(),
    })?;

    let upstream_oid = upstream.get().target().ok_or_else(|| DevBaseError::Scan {
        message: "No upstream target".to_string(),
    })?;

    let (ahead, behind) = repo.graph_ahead_behind(local_oid, upstream_oid)
        .map_err(|e| DevBaseError::Scan {
            message: format!("Failed to compute ahead/behind: {e}"),
        })?;

    Ok((ahead as u32, behind as u32))
}

/// Status summary for display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RepoStatus {
    /// All clean, up to date
    Clean,
    /// Has uncommitted changes
    Dirty,
    /// Has commits to push
    Ahead,
    /// Has commits to pull
    Behind,
    /// Has both pushes and pulls needed
    Diverged,
}

impl RepoHealth {
    /// Get the overall status for display.
    pub fn status(&self) -> RepoStatus {
        if self.commits_ahead > 0 && self.commits_behind > 0 {
            RepoStatus::Diverged
        } else if self.commits_behind > 0 {
            RepoStatus::Behind
        } else if self.commits_ahead > 0 {
            RepoStatus::Ahead
        } else if self.is_dirty {
            RepoStatus::Dirty
        } else {
            RepoStatus::Clean
        }
    }
}
