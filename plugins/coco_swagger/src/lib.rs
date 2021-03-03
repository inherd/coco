use core_model::CocoConfig;
use core_model::PluginInterface;

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

impl Default for CocoSwagger {
    fn default() -> Self {
        CocoSwagger {}
    }
}

#[no_mangle]
pub fn plugin() -> Box<dyn PluginInterface> {
    Box::new(CocoSwagger::default())
}
