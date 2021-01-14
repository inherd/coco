pub use git_repository::GitRepository;

pub mod git_branch;
pub mod git_repository;

#[cfg(test)]
mod test {
    use crate::infrastructure::git::git_branch::GitBranch;
    use crate::infrastructure::git::GitRepository;
    use std::path::Path;
    use std::sync::Once;

    static INIT: Once = Once::new();

    pub fn initialize() {
        INIT.call_once(|| {
            GitRepository::clone("https://github.com/inherd/coco.fixtures");
        });
    }

    #[test]
    fn should_verify_github_dir() {
        initialize();
        let repo = GitRepository::clone("https://github.com/inherd/coco.fixtures");
        let path_str = repo.path().to_str().unwrap();
        let path = Path::new("github.com/inherd/coco.fixtures");
        assert!(path_str.contains(path.to_str().unwrap()));
    }

    #[test]
    fn should_list_branch() {
        initialize();

        let repo = GitRepository::clone("https://github.com/inherd/coco.fixtures");
        let branches = GitBranch::list(repo);
        assert_eq!(5, branches.len());
    }

    #[test]
    fn should_get_master() {
        initialize();

        let repo = GitRepository::clone("https://github.com/inherd/coco.fixtures");
        let branch = GitBranch::get("master", repo).unwrap();
        assert_eq!("master", branch.name);
        assert_eq!(1610519809, branch.first_commit_date);
        assert_eq!("Phodal Huang", branch.author);
        assert_eq!("GitHub", branch.committer);

        let first_second_commit_duration_hours = 6;
        let hours = branch.duration / 3600;

        assert!(hours >= first_second_commit_duration_hours);
    }
}
