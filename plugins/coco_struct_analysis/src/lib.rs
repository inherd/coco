use core_model::CocoConfig;
use plugin_interface::PluginInterface;

pub struct CocoStructAnslysis {}

impl PluginInterface for CocoStructAnslysis {
    fn name(&self) -> &'static str {
        "coco.struct_analysis"
    }

    fn on_plugin_load(&self) {}

    fn on_plugin_unload(&self) {}

    fn execute(&self, config: CocoConfig) {
        println!("{:?}", config);
    }
}

impl Default for CocoStructAnslysis {
    fn default() -> Self {
        CocoStructAnslysis {}
    }
}

#[no_mangle]
pub fn plugin() -> Box<dyn PluginInterface> {
    Box::new(CocoStructAnslysis::default())
}
