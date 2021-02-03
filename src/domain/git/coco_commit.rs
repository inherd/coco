use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CocoCommit {
    pub branch: String,
    pub commit_id: String,
    pub author: String,
    pub committer: String,
    pub date: i64,
    pub message: String,
    pub changes: Vec<FileChange>,
    pub parent_hashes: Vec<String>,
    pub tree_hash: String,
}

impl Default for CocoCommit {
    fn default() -> Self {
        CocoCommit {
            branch: "".to_string(),
            commit_id: "".to_string(),
            author: "".to_string(),
            committer: "".to_string(),
            date: 0,
            message: "".to_string(),
            changes: vec![],
            parent_hashes: vec![],
            tree_hash: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileChange {
    pub added: i32,
    pub deleted: i32,
    pub file: String,
    pub mode: String,
}
