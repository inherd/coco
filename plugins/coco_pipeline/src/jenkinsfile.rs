use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JenkinsFile {
    pub name: String,
    pub stages: Vec<JenkinsStage>,
    pub post: Vec<PostConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JenkinsStage {
    pub name: String,
    pub jobs: Vec<JenkinsJob>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JenkinsJob {
    pub name: String,
    pub job: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostConfig {
    pub key: String,
    pub value: Vec<JenkinsJob>,
}
