use crate::domain::git::CocoCommit;
use crate::infrastructure::git::git_command::get_commit_message;
use crate::infrastructure::git::git_log_parser::GitMessageParser;
use crate::infrastructure::url_format;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShortCommit {
    pub branch: String,
    pub commit_id: String,
    pub author: String,
    pub date: i64,
    pub message: String,
    pub parent_hashes: Vec<String>,
    pub tree_hash: String,
}

impl ShortCommit {
    pub fn convert(commit: CocoCommit) -> ShortCommit {
        Self {
            branch: commit.branch,
            commit_id: commit.commit_sha,
            author: commit.author,
            date: commit.date,
            message: commit.message,
            parent_hashes: commit.parent_hashes,
            tree_hash: commit.tree_hash,
        }
    }
}

pub fn analysis(url: &str) -> Vec<ShortCommit> {
    let local_path = url_format::uri_to_path(url);

    let messages = get_commit_message(Some(format!("{}", local_path.display())));
    let vec = GitMessageParser::parse(messages.as_str());

    let mut results = vec![];
    for commit in vec {
        results.push(ShortCommit::convert(commit))
    }

    return results;
}
