use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub fist_commit: String,
    pub last_commit: String,
    pub duration: String,
    pub author: String,
}
