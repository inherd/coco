use crate::app::format_branch::FormatBranch;
use crate::infrastructure::git::git_branch::GitBranch;
use crate::infrastructure::git::GitRepository;

pub fn branches_info(url: &str) -> String {
    let repo = GitRepository::clone(url);
    let mut branches = vec![];

    for br in GitBranch::list(repo) {
        branches.push(FormatBranch::from(br));
    }

    let branches_info = serde_json::to_string_pretty(&branches).unwrap();
    return branches_info;
}

#[cfg(test)]
mod test {
    use crate::app::format_branch::FormatBranch;
    use crate::domain::git::coco_branch::CocoBranch;

    #[test]
    fn should_output_really_date() {
        let branch = FormatBranch::from(CocoBranch {
            name: "master".to_string(),
            first_commit_date: 1610519809,
            last_commit_date: 1610541520,
            duration: 21711,
            commits_count: 0,
            author: "GitHub".to_string(),
            committer: "Phodal HUANG".to_string(),
        });

        assert_eq!("2021-01-13 06:36:49", branch.first_commit_str);
        assert_eq!("2021-01-13 12:38:40", branch.last_commit_str);
    }
}
