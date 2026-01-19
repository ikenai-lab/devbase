//! Tests for git module.

#[cfg(test)]
mod tests {
    use super::super::*;
    use std::fs;
    use std::process::Command;
    use tempfile::tempdir;

    fn init_git_repo(path: &std::path::Path) {
        Command::new("git")
            .args(["init"])
            .current_dir(path)
            .output()
            .expect("Failed to init git repo");
        
        // Configure git user for commits
        Command::new("git")
            .args(["config", "user.email", "test@test.com"])
            .current_dir(path)
            .output()
            .ok();
        
        Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(path)
            .output()
            .ok();
    }

    #[test]
    fn test_clean_repo_status() {
        let temp = tempdir().unwrap();
        init_git_repo(temp.path());

        // Make an initial commit
        fs::write(temp.path().join("README.md"), "# Test").unwrap();
        Command::new("git")
            .args(["add", "."])
            .current_dir(temp.path())
            .output()
            .ok();
        Command::new("git")
            .args(["commit", "-m", "initial"])
            .current_dir(temp.path())
            .output()
            .ok();

        let health = get_repo_health(temp.path()).unwrap();
        
        assert!(!health.is_dirty);
        assert_eq!(health.uncommitted_count, 0);
        assert_eq!(health.status(), RepoStatus::Clean);
    }

    #[test]
    fn test_dirty_repo_status() {
        let temp = tempdir().unwrap();
        init_git_repo(temp.path());

        // Create uncommitted file
        fs::write(temp.path().join("dirty.txt"), "uncommitted").unwrap();

        let health = get_repo_health(temp.path()).unwrap();
        
        assert!(health.is_dirty);
        assert!(health.uncommitted_count > 0);
        assert_eq!(health.status(), RepoStatus::Dirty);
    }

    #[test]
    fn test_staged_files_count() {
        let temp = tempdir().unwrap();
        init_git_repo(temp.path());

        // Create and stage a file
        fs::write(temp.path().join("staged.txt"), "staged content").unwrap();
        Command::new("git")
            .args(["add", "staged.txt"])
            .current_dir(temp.path())
            .output()
            .ok();

        let health = get_repo_health(temp.path()).unwrap();
        
        assert!(health.is_dirty);
        assert!(health.staged_count > 0);
    }

    #[test]
    fn test_current_branch() {
        let temp = tempdir().unwrap();
        init_git_repo(temp.path());

        // Make initial commit to create branch
        fs::write(temp.path().join("README.md"), "# Test").unwrap();
        Command::new("git")
            .args(["add", "."])
            .current_dir(temp.path())
            .output()
            .ok();
        Command::new("git")
            .args(["commit", "-m", "init"])
            .current_dir(temp.path())
            .output()
            .ok();

        let health = get_repo_health(temp.path()).unwrap();
        
        // Branch is either main or master depending on git config
        assert!(
            health.current_branch == Some("main".to_string()) ||
            health.current_branch == Some("master".to_string())
        );
    }

    #[test]
    fn test_stash_count() {
        let temp = tempdir().unwrap();
        init_git_repo(temp.path());

        // Make initial commit
        fs::write(temp.path().join("README.md"), "# Test").unwrap();
        Command::new("git")
            .args(["add", "."])
            .current_dir(temp.path())
            .output()
            .ok();
        Command::new("git")
            .args(["commit", "-m", "init"])
            .current_dir(temp.path())
            .output()
            .ok();

        // Create and stash changes
        fs::write(temp.path().join("stashed.txt"), "to stash").unwrap();
        Command::new("git")
            .args(["add", "."])
            .current_dir(temp.path())
            .output()
            .ok();
        Command::new("git")
            .args(["stash"])
            .current_dir(temp.path())
            .output()
            .ok();

        let health = get_repo_health(temp.path()).unwrap();
        
        assert_eq!(health.stash_count, 1);
    }
}
