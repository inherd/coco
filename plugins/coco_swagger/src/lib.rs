use core_model::CocoConfig;
use plugin_interface::PluginInterface;

pub struct CocoSwagger {}

impl PluginInterface for CocoSwagger {
    fn name(&self) -> &'static str {
        "coco.swagger"
    }

    fn on_plugin_load(&self) {}

    fn on_plugin_unload(&self) {}

    fn execute(&self, config: CocoConfig) {
        println!("{:?}", config);
    }
}
