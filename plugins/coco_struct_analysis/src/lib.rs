pub mod cmd_ctags;
pub mod option_ctags;

use core_model::CocoConfig;
use plugin_interface::PluginInterface;

pub struct CocoStructAnalysis {}

impl PluginInterface for CocoStructAnalysis {
    fn name(&self) -> &'static str {
        "coco.struct_analysis"
    }

    fn on_plugin_load(&self) {}

    fn on_plugin_unload(&self) {}

    fn execute(&self, config: CocoConfig) {
        println!("{:?}", config);
    }
}

impl Default for CocoStructAnalysis {
    fn default() -> Self {
        CocoStructAnalysis {}
    }
}

#[no_mangle]
pub fn plugin() -> Box<dyn PluginInterface> {
    Box::new(CocoStructAnalysis::default())
}
