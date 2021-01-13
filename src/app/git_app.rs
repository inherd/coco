use crate::infrastructure::git::git_branch::GitBranch;
use crate::infrastructure::git::GitRepository;

pub fn get_repo(url: &str) -> String {
    let repo = GitRepository::clone(url);
    let branches = GitBranch::list(repo);

    let output = serde_json::to_string(&branches).unwrap();
    return output;
}
