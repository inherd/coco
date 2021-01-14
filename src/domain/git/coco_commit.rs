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
    pub added: int,
    pub deleted: int,
    pub file: String,
    pub mode: String
}
