use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CocoBranch {
    pub name: String,
    pub first_commit_date: i64,
    pub last_commit_date: i64,
    pub duration: i64,
    pub author: String,
    pub committer: String,
}

impl CocoBranch {
    pub fn new(name: &str) -> CocoBranch {
        CocoBranch {
            name: name.to_string(),
            first_commit_date: 0,
            last_commit_date: 0,
            duration: 0,
            author: "".to_string(),
            committer: "".to_string(),
        }
    }
}
