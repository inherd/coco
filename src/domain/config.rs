use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CocoConfig {
    pub repo: Vec<RepoConfig>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RepoConfig {
    pub url: String,
}

