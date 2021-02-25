// extern crate pest;
// #[macro_use]
// extern crate pest_derive;

pub mod github_action;
pub mod jenkinsfile;
pub mod pipeline;

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
        println!("{:?}", config);
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
