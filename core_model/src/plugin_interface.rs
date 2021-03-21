use crate::CocoConfig;

pub trait PluginInterface {
    /// name of plugins
    /// should start with `coco.`, such as `coco.swagger`
    fn name(&self) -> &'static str;
    /// event for load plugin
    fn on_plugin_load(&self) {}
    /// event of unload plugin
    fn on_plugin_unload(&self) {}
    /// execute plugin
    fn execute(&self, config: CocoConfig);
}
