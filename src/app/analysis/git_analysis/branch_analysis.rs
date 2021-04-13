use super::FormatBranch;
use crate::infrastructure::git::git_branch::GitBranch;
use crate::infrastructure::git::GitRepository;

pub fn analysis(url: &str, local_git: bool) -> Vec<FormatBranch> {
    let repo = GitRepository::open(url, local_git);

    let mut branches = vec![];
    for br in GitBranch::list(repo) {
        branches.push(FormatBranch::from(br));
    }

    return branches;
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore]
    #[test]
    fn local_project_test() {
        let branches = analysis(".", false);
        assert!(branches.len() >= 2);
    }
}
