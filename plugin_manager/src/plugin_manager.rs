use dlopen::wrapper::{Container, WrapperApi};

use plugin_interface::PluginInterface;
use std::path::PathBuf;

#[derive(WrapperApi)]
struct Wrapper {
    plugin: fn() -> Box<dyn PluginInterface>,
}

pub struct PluginManager {}

impl PluginManager {
    #[allow(dead_code)]
    fn run() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let plugin_path = "target/debug/libcoco_swagger.dylib";
        let path = root.parent().unwrap().join(plugin_path);

        let cont: Container<Wrapper> =
            unsafe { Container::load(path) }.expect("Could not open library or load symbols");

        let plugin = cont.plugin();
        println!("{:?}", plugin.name());
    }
}

#[cfg(test)]
mod tests {
    use crate::plugin_manager::PluginManager;

    #[ignore]
    #[test]
    fn test_plugin_run_in_local() {
        PluginManager::run();
    }
}
