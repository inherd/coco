use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CocoCommit {
    pub branch: String,
    pub rev: String,
    pub author: String,
    pub committer: String,
    pub date: i64,
    pub message: String,
    pub changes: Vec<FileChange>,
}

impl CocoCommit {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileChange {
    pub added: i64,
    pub deleted: i64,
    pub file: String,
    pub mode: String,
}
