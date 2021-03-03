pub mod coco_pipeline;
pub mod coco_pipeline_plugin;
pub mod github_action;
pub mod jenkinsfile;

use core_model::CocoConfig;
use core_model::PluginInterface;

pub struct CocoPipeline {}

impl PluginInterface for CocoPipeline {
    fn name(&self) -> &'static str {
        "coco.pipeline"
    }

    fn on_plugin_load(&self) {}

    fn on_plugin_unload(&self) {}

    fn execute(&self, config: CocoConfig) {
        coco_pipeline_plugin::execute(config);
    }
}

impl Default for CocoPipeline {
    fn default() -> Self {
        CocoPipeline {}
    }
}

#[no_mangle]
pub fn plugin() -> Box<dyn PluginInterface> {
    Box::new(CocoPipeline::default())
}

#[cfg(test)]
mod tests {
    use crate::coco_pipeline::CocoPipeline;
    use crate::coco_pipeline_plugin::execute;
    use core_model::{CocoConfig, RepoConfig};
    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;

    pub fn fixtures_dir() -> PathBuf {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();
        let ctags_dir = root_dir
            .clone()
            .join("_fixtures")
            .join("pipeline")
            .join("jenkinsfile");

        return ctags_dir;
    }

    #[test]
    fn should_run_pipeline_analysis() {
        let mut repos = vec![];
        repos.push(RepoConfig {
            url: format!("{}", fixtures_dir().display()),
            languages: None,
        });
        let config = CocoConfig {
            repos,
            plugins: None,
        };

        execute(config);

        let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(".coco")
            .join("reporter")
            .join("pipeline");
        let output_dir = base_dir.join("jenkinsfile.json");

        let mut file = File::open(output_dir).unwrap();
        let mut code = String::new();
        file.read_to_string(&mut code).unwrap();
        let pipelines: Vec<CocoPipeline> = serde_json::from_str(&code).unwrap();

        assert_eq!(1, pipelines.len());
        assert_eq!(5, pipelines[0].stages.len());
        // assert_eq!(2, pipelines[0].stages[0].sub_stages.len());
    }
}
