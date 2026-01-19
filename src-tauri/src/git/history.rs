use crate::error::Result;
use git2::{Repository, Sort};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommitLogEntry {
    pub oid: String,
    pub short_oid: String,
    pub message: String,
    pub author_name: String,
    pub author_email: String,
    pub date: i64, // Unix timestamp
    pub parents: Vec<String>,
    pub refs: Vec<String>, // Branches/Tags pointing here
}

pub fn get_repo_history(path: &Path, limit: usize) -> Result<Vec<CommitLogEntry>> {
    let repo = Repository::open(path)?;
    let mut revwalk = repo.revwalk()?;
    
    // Sort topologically (parents before children) and by time
    revwalk.set_sorting(Sort::TOPOLOGICAL | Sort::TIME)?;
    
    // Push HEAD
    if let Ok(head) = repo.head() {
        if let Some(oid) = head.target() {
            revwalk.push(oid)?;
        }
    } else {
        // Empty repo or no HEAD
        return Ok(Vec::new());
    }

    let mut commits = Vec::new();
    let mut count = 0;

    for oid_result in revwalk {
        if count >= limit {
            break;
        }
        let oid = oid_result?;
        let commit = repo.find_commit(oid)?;
        
        // Extract basic info
        let message = commit.summary().unwrap_or("").to_string();
        let author = commit.author();
        let author_name = author.name().unwrap_or("Unknown").to_string();
        let author_email = author.email().unwrap_or("").to_string();
        let date = commit.time().seconds();
        
        let parents = commit.parent_ids()
            .map(|id| id.to_string())
            .collect();
            
        // TODO: Efficiently finding refs for each commit is expensive (O(N*M))
        // For now, we will leave refs empty or implement a lookup map separately if needed for graph.
        
        commits.push(CommitLogEntry {
            oid: oid.to_string(),
            short_oid: oid.to_string()[0..7].to_string(),
            message,
            author_name,
            author_email,
            date,
            parents,
            refs: Vec::new(),
        });
        
        count += 1;
    }

    Ok(commits)
}
