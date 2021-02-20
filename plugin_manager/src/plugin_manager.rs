use dlopen::wrapper::{Container, WrapperApi};

use plugin_interface::PluginInterface;
use std::path::PathBuf;

#[derive(WrapperApi)]
struct Wrapper {
    plugin: fn() -> Box<dyn PluginInterface>,
}

pub struct PluginManager {}

impl PluginManager {
    pub fn run(plugin_name: &str) {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let plugin_path = format!("target/debug/libcoco_{}.dylib", plugin_name);
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
        PluginManager::run("swagger");
    }
}
