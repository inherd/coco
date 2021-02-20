use serde::{Deserialize, Serialize};

/// Coco Config from `coco.yml`
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CocoConfig {
    pub repo: Vec<RepoConfig>,
    pub plugins: Vec<String>,
}

impl Default for CocoConfig {
    fn default() -> Self {
        CocoConfig {
            repo: vec![],
            plugins: vec![],
        }
    }
}

/// RepoConfig
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RepoConfig {
    pub url: String,
}
