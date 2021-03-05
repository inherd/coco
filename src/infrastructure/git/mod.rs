pub use git_repository::GitRepository;

pub mod cmd_git;
pub mod git_branch;
pub mod git_commit_message;
pub mod git_file_history;
pub mod git_log_parser;
pub mod git_repository;
pub mod git_tag_parser;

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use std::sync::Once;

    use crate::infrastructure::git::cmd_git::commit_message;
    use crate::infrastructure::git::git_branch::GitBranch;
    use crate::infrastructure::git::git_log_parser::GitMessageParser;
    use crate::infrastructure::git::{git_file_history, GitRepository};
    use core_model::url_format;

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
        assert!(branches.len() >= 5);
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

    #[test]
    fn should_summary_all_commits() {
        initialize();

        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let local_path = url_format::uri_to_path("https://github.com/coco-rs/coco.fixtures");
        let abs_path = root.join(local_path);

        let messages = commit_message(Some(format!("{}", abs_path.display())));
        let vec = GitMessageParser::parse(messages.as_str());
        assert!(vec.len() >= 3);

        let first = &vec[0];
        assert_eq!("Initial commit", first.message);
        assert_eq!(3, first.changes.len());
        assert_eq!("origin/gh-pages", first.branch);
    }

    #[test]
    fn should_get_file_history() {
        initialize();

        let root = url_format::uri_to_path("https://github.com/coco-rs/coco.fixtures");
        let tree = git_file_history::by_path(root, 1.0);

        let name = tree.get_children()[0].name();
        assert_eq!("LICENSE", name.to_str().unwrap());
    }
}
