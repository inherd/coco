use crate::app::git_analysis::FormatBranch;
use crate::infrastructure::git::git_branch::GitBranch;
use crate::infrastructure::git::git_command::get_commit_message;
use crate::infrastructure::git::git_log_parser::GitMessageParser;
use crate::infrastructure::git::GitRepository;
use crate::infrastructure::url_format;

pub fn analysis(url: &str) -> Vec<FormatBranch> {
    let repo = GitRepository::open(url);

    let mut branches = vec![];
    for br in GitBranch::list(repo) {
        branches.push(FormatBranch::from(br));
    }

    let local_path = url_format::uri_to_path(url);

    let messages = get_commit_message(Some(format!("{}", local_path.display())));
    let _vec = GitMessageParser::parse(messages.as_str());

    // println!("{:?}", vec);
    return branches;
}

#[cfg(test)]
mod test {
    use crate::app::git_analysis::analysis;

    #[ignore]
    #[test]
    fn local_project_test() {
        let branches = analysis(".");
        assert!(branches.len() >= 2);
    }
}
