use serde::{Deserialize, Serialize};

use crate::infrastructure::time_format::format_unix_time;

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormatBranch {
    pub name: String,
    pub author: String,
    pub committer: String,
    pub first_commit_str: String,
    pub last_commit_str: String,
    pub first_commit_date: i64,
    pub last_commit_date: i64,
}

impl FormatBranch {
    pub fn from(br: CocoBranch) -> FormatBranch {
        FormatBranch {
            name: br.name,
            author: br.author,
            committer: br.committer,

            first_commit_str: format_unix_time(br.first_commit_date),
            last_commit_str: format_unix_time(br.last_commit_date),
            first_commit_date: br.first_commit_date,
            last_commit_date: br.last_commit_date,
        }
    }
}
