use super::FormatBranch;
use crate::infrastructure::git::git_branch::GitBranch;
use crate::infrastructure::git::GitRepository;

pub fn analysis(url: &str) -> Vec<FormatBranch> {
    let repo = GitRepository::open(url);

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
        let branches = analysis(".");
        assert!(branches.len() >= 2);
    }
}
