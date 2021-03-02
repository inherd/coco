use jenkinsfile::Jenkinsfile;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pipeline {
    pub name: String,
    pub stages: Vec<PipelineStage>,
}

impl Pipeline {
    pub fn from(jenkinsfile: Jenkinsfile) -> Pipeline {
        let mut pipeline = Pipeline {
            name: jenkinsfile.name,
            stages: vec![],
        };

        for main_stage in &jenkinsfile.stages {
            let mut stage = PipelineStage::new(main_stage.name.clone());
            stage.steps = main_stage.steps.clone();
            for sub_stage in &main_stage.sub_stages {
                stage.sub_stages.push(PipelineStage {
                    name: sub_stage.name.clone(),
                    steps: sub_stage.steps.clone(),
                    is_parallel: false,
                    sub_stages: vec![],
                })
            }

            pipeline.stages.push(stage);
        }

        pipeline
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PipelineStage {
    pub name: String,
    pub steps: Vec<String>,
    pub is_parallel: bool,
    pub sub_stages: Vec<PipelineStage>,
}

impl PipelineStage {
    pub fn new(name: String) -> PipelineStage {
        Self {
            name,
            steps: vec![],
            is_parallel: false,
            sub_stages: vec![],
        }
    }
}

impl Default for PipelineStage {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            steps: vec![],
            is_parallel: false,
            sub_stages: vec![],
        }
    }
}
