use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CocoConfig {
    pub repo: Vec<RepoConfig>,
}

impl Default for CocoConfig {
    fn default() -> Self {
        CocoConfig { repo: vec![] }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RepoConfig {
    pub url: String,
}
