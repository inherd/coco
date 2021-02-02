pub use git_repository::GitRepository;

pub mod git_branch;
pub mod git_command;
pub mod git_commit_message;
pub mod git_log_parser;
pub mod git_repository;

#[cfg(test)]
mod test {
    use crate::infrastructure::git::git_branch::GitBranch;
    use crate::infrastructure::git::GitRepository;
    use std::sync::Once;

    static INIT: Once = Once::new();

    pub fn initialize() {
        INIT.call_once(|| {
            GitRepository::open("https://github.com/coco-rs/coco.fixtures");
        });
    }

    #[test]
    fn should_verify_github_dir() {
        initialize();
        let repo = GitRepository::open("https://github.com/coco-rs/coco.fixtures");
        let path_str: &str = repo.path().to_str().unwrap();

        assert!(path_str.contains("github.com"));
        assert!(path_str.contains("coco-rs"));
        assert!(path_str.contains("coco.fixtures"));
    }

    #[test]
    fn should_list_branch() {
        initialize();

        let repo = GitRepository::open("https://github.com/coco-rs/coco.fixtures");
        let branches = GitBranch::list(repo);
        assert!(branches.len() > 5);
    }

    #[test]
    fn should_get_master() {
        initialize();

        let repo = GitRepository::open("https://github.com/coco-rs/coco.fixtures");
        let branch = GitBranch::get("master", repo).unwrap();
        assert_eq!("Local", branch.branch_type);
        assert_eq!("master", branch.name);
        assert_eq!(1610519809, branch.first_commit_date);

        let first_second_commit_duration_hours = 6;
        let hours = branch.duration / 3600;

        assert!(hours >= first_second_commit_duration_hours);
    }

    #[test]
    fn should_count_master_commits() {
        initialize();

        let repo = GitRepository::open("https://github.com/coco-rs/coco.fixtures");
        let branch = GitBranch::get("master", repo).unwrap();

        assert!(branch.commits_count >= 2);
    }
}
