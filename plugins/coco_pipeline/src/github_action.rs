use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitHubAction {
    pub builds: Vec<GithubBuild>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubBuild {
    pub name: String,
    pub jobs: Vec<GitHubJob>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitHubJob {
    pub name: String,
}
