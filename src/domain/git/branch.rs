use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Branch {
    pub name: String,
    pub fist_commit_date: String,
    pub last_commit_date: String,
    pub duration: String,
    pub author: String,
    pub committer: String,
    // todo: add branch type support
}

impl Branch {
    pub fn new(name: &str) -> Branch {
        Branch {
            name: name.to_string(),
            fist_commit_date: "".to_string(),
            last_commit_date: "".to_string(),
            duration: "".to_string(),
            author: "".to_string(),
            committer: "".to_string(),
        }
    }
}
