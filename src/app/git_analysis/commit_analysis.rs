use crate::domain::git::CocoCommit;
use crate::infrastructure::git::git_command::get_commit_message;
use crate::infrastructure::git::git_log_parser::GitMessageParser;
use crate::infrastructure::url_format;

pub fn analysis(url: &str) -> Vec<CocoCommit> {
    let local_path = url_format::uri_to_path(url);

    let messages = get_commit_message(Some(format!("{}", local_path.display())));
    let vec = GitMessageParser::parse(messages.as_str());

    return vec;
}
