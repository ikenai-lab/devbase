//! Tests for scanner module.

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
    }

    #[test]
    fn test_find_single_repo() {
        let temp = tempdir().unwrap();
        let repo_path = temp.path().join("my-repo");
        fs::create_dir_all(&repo_path).unwrap();
        init_git_repo(&repo_path);

        let repos = find_git_repos(temp.path(), 5).unwrap();
        
        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].name, "my-repo");
    }

    #[test]
    fn test_find_multiple_repos() {
        let temp = tempdir().unwrap();
        
        for name in &["repo1", "repo2", "repo3"] {
            let repo_path = temp.path().join(name);
            fs::create_dir_all(&repo_path).unwrap();
            init_git_repo(&repo_path);
        }

        let repos = find_git_repos(temp.path(), 5).unwrap();
        
        assert_eq!(repos.len(), 3);
    }

    #[test]
    fn test_find_nested_repos() {
        let temp = tempdir().unwrap();
        
        let deep_path = temp.path().join("level1").join("level2").join("deep-repo");
        fs::create_dir_all(&deep_path).unwrap();
        init_git_repo(&deep_path);

        let repos = find_git_repos(temp.path(), 10).unwrap();
        
        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].name, "deep-repo");
    }

    #[test]
    fn test_max_depth_limits_search() {
        let temp = tempdir().unwrap();
        
        // Create a deep repo
        let deep_path = temp.path().join("a").join("b").join("c").join("deep");
        fs::create_dir_all(&deep_path).unwrap();
        init_git_repo(&deep_path);

        // Shallow search should not find it
        let repos = find_git_repos(temp.path(), 2).unwrap();
        assert_eq!(repos.len(), 0);

        // Deep search should find it
        let repos = find_git_repos(temp.path(), 10).unwrap();
        assert_eq!(repos.len(), 1);
    }

    #[test]
    fn test_is_git_repo() {
        let temp = tempdir().unwrap();
        let repo_path = temp.path().join("repo");
        fs::create_dir_all(&repo_path).unwrap();
        
        assert!(!is_git_repo(&repo_path));
        
        init_git_repo(&repo_path);
        
        assert!(is_git_repo(&repo_path));
    }

    #[test]
    fn test_scan_nonexistent_path() {
        let result = find_git_repos(std::path::Path::new("/nonexistent/path"), 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_org_from_ssh_url() {
        let url = "git@github.com:myorg/myrepo.git";
        assert_eq!(extract_org_from_url(url), Some("myorg".to_string()));
    }

    #[test]
    fn test_extract_org_from_https_url() {
        let url = "https://github.com/myorg/myrepo.git";
        assert_eq!(extract_org_from_url(url), Some("myorg".to_string()));
    }

    #[test]
    fn test_extract_host_from_ssh_url() {
        let url = "git@github.com:owner/repo.git";
        assert_eq!(extract_host_from_url(url), Some("github.com".to_string()));
    }

    #[test]
    fn test_extract_host_from_https_url() {
        let url = "https://gitlab.com/owner/repo.git";
        assert_eq!(extract_host_from_url(url), Some("gitlab.com".to_string()));
    }
}
