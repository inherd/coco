pub use git_repository::GitRepository;

pub mod git_branch;
pub mod git_repository;

#[cfg(test)]
mod test {
    use crate::infrastructure::git::git_branch::GitBranch;
    use crate::infrastructure::git::GitRepository;
    use std::sync::Once;

    static INIT: Once = Once::new();

    pub fn initialize() {
        INIT.call_once(|| {
            GitRepository::clone("https://github.com/phodal/coco.fixtures");
        });
    }

    #[test]
    fn should_list_branch() {
        initialize();

        let repo = GitRepository::clone("https://github.com/phodal/coco.fixtures");
        let branches = GitBranch::list(repo);
        assert_eq!(5, branches.len());
    }

    #[test]
    fn should_get_master() {
        initialize();

        let repo = GitRepository::clone("https://github.com/phodal/coco.fixtures");
        let branch = GitBranch::get("master", repo).unwrap();
        assert_eq!("master", branch.name);
        assert_eq!("1610519809", branch.date);
    }
}
