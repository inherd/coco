use jenkinsfile::Jenkinsfile;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CocoPipeline {
    pub name: String,
    pub stages: Vec<CocoPipelineStage>,
}

impl CocoPipeline {
    pub fn from(jenkinsfile: Jenkinsfile) -> CocoPipeline {
        let mut pipeline = CocoPipeline {
            name: jenkinsfile.name,
            stages: vec![],
        };

        for main_stage in &jenkinsfile.stages {
            let mut stage = CocoPipelineStage::new(main_stage.name.clone());
            stage.steps = main_stage.steps.clone();
            for sub_stage in &main_stage.sub_stages {
                stage.sub_stages.push(CocoPipelineStage {
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
pub struct CocoPipelineStage {
    pub name: String,
    pub steps: Vec<String>,
    pub is_parallel: bool,
    pub sub_stages: Vec<CocoPipelineStage>,
}

impl CocoPipelineStage {
    pub fn new(name: String) -> CocoPipelineStage {
        Self {
            name,
            steps: vec![],
            is_parallel: false,
            sub_stages: vec![],
        }
    }
}

impl Default for CocoPipelineStage {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            steps: vec![],
            is_parallel: false,
            sub_stages: vec![],
        }
    }
}
