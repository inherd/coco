use crate::app::git_analysis::FormatBranch;
use crate::infrastructure::git::git_branch::GitBranch;
use crate::infrastructure::git::GitRepository;

pub fn branches_info(url: &str) -> String {
    let repo = GitRepository::open(url);
    let mut branches = vec![];

    for br in GitBranch::list(repo) {
        branches.push(FormatBranch::from(br));
    }

    let branches_info = serde_json::to_string_pretty(&branches).unwrap();
    return branches_info;
}
