use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CocoBranch {
    pub name: String,
    pub branch_type: String,
    pub first_commit_date: i64,
    pub last_commit_date: i64,
    pub duration: i64,
    pub commits_count: usize,
    pub commits: Vec<String>,
    pub latest_changeset: String,
}

impl CocoBranch {
    pub fn new(name: &str) -> CocoBranch {
        CocoBranch {
            name: name.to_string(),
            branch_type: "".to_string(),
            first_commit_date: 0,
            last_commit_date: 0,
            duration: 0,
            commits_count: 0,
            commits: vec![],
            latest_changeset: "".to_string(),
        }
    }
}
