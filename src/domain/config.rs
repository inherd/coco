use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CocoConfig {
    pub repo: Vec<RepoConfig>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RepoConfig {
    pub url: String,
}
