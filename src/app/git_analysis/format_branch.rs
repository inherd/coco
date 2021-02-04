use serde::{Deserialize, Serialize};

use crate::domain::git::CocoBranch;
use crate::infrastructure::time_format::format_unix_time;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormatBranch {
    pub name: String,
    pub first_commit_str: String,
    pub last_commit_str: String,
    pub first_commit_date: i64,
    pub last_commit_date: i64,
    pub commits: Vec<String>,
    pub latest_changeset: String,
}

impl FormatBranch {
    pub fn from(br: CocoBranch) -> FormatBranch {
        FormatBranch {
            name: br.name,
            first_commit_str: format_unix_time(br.first_commit_date),
            last_commit_str: format_unix_time(br.last_commit_date),
            first_commit_date: br.first_commit_date,
            last_commit_date: br.last_commit_date,
            commits: br.commits,
            latest_changeset: br.latest_changeset,
        }
    }
}
