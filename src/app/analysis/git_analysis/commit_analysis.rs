use crate::domain::git::CocoCommit;
use crate::infrastructure::git::cmd_git::commit_message;
use crate::infrastructure::git::git_log_parser::GitMessageParser;
use core_model::url_format;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShortCommit {
    pub branch: String,
    pub commit_id: String,
    pub author: String,
    pub email: String,
    pub date: i64,
    pub message: String,
    pub parent_hashes: Vec<String>,
    pub tree_hash: String,
    pub total_added: i32,
    pub total_deleted: i32,
    pub changed_file_count: i32,
}

impl ShortCommit {
    pub fn convert(commit: CocoCommit) -> ShortCommit {
        Self {
            branch: commit.branch,
            commit_id: commit.commit_id,
            author: commit.author,
            email: commit.email,
            date: commit.date,
            message: commit.message,
            parent_hashes: commit.parent_hashes,
            tree_hash: commit.tree_hash,
            total_added: commit.total_added,
            total_deleted: commit.total_deleted,
            changed_file_count: commit.changed_file_count,
        }
    }
}

pub fn analysis(url: &str) -> Vec<ShortCommit> {
    let local_path = url_format::uri_to_path(url);

    let messages = commit_message(Some(format!("{}", local_path.display())));
    let vec = GitMessageParser::parse(messages.as_str());

    let mut results = vec![];
    for commit in vec {
        results.push(ShortCommit::convert(commit))
    }

    return results;
}
