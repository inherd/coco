pub mod github_action;
pub mod jenkinsfile;
pub mod pipeline;
pub mod pipeline_plugin;

use core_model::CocoConfig;
use plugin_interface::PluginInterface;

pub struct CocoPipeline {}

impl PluginInterface for CocoPipeline {
    fn name(&self) -> &'static str {
        "coco.pipeline"
    }

    fn on_plugin_load(&self) {}

    fn on_plugin_unload(&self) {}

    fn execute(&self, config: CocoConfig) {
        pipeline_plugin::execute(config);
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
    use crate::pipeline_plugin::execute;
    use core_model::{CocoConfig, RepoConfig};
    use std::path::PathBuf;

    pub fn ctags_fixtures_dir() -> PathBuf {
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
            url: format!("{}", ctags_fixtures_dir().display()),
            languages: None,
        });
        let config = CocoConfig {
            repos: repos,
            plugins: None,
        };

        execute(config);
    }
}
