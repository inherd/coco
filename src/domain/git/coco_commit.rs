use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CocoCommit {
    pub rev: String,
    pub author: String,
    pub date: String,
    pub message: String,
    pub changes: Vec<FileChange>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileChange {
    pub added: i64,
    pub deleted: i64,
    pub file: String,
    pub mode: String
}
