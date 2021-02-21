use dlopen::wrapper::{Container, WrapperApi};

use plugin_interface::PluginInterface;
use std::path::PathBuf;

const BUILD_TYPE: &str = if cfg!(debug_assertions) {
    "debug"
} else {
    "release"
};

#[derive(WrapperApi)]
struct Wrapper {
    plugin: fn() -> Box<dyn PluginInterface>,
}

pub struct PluginManager {}

impl PluginManager {
    pub fn plugin(plugin_name: &str) -> Box<dyn PluginInterface> {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let plugin_path = Self::get_plugin_path(plugin_name);
        let path = root.parent().unwrap().join(plugin_path);

        let cont: Container<Wrapper> =
            unsafe { Container::load(path) }.expect("Could not open library or load symbols");

        return cont.plugin();
    }

    #[cfg(target_os = "linux")]
    fn get_plugin_path(plugin_name: &str) -> String {
        format!("target/{}/libcoco_{}.so", BUILD_TYPE, plugin_name)
    }

    #[cfg(target_os = "macos")]
    fn get_plugin_path(plugin_name: &str) -> String {
        format!("target/{}/libcoco_{}.dylib", BUILD_TYPE, plugin_name)
    }

    #[cfg(target_os = "windows")]
    fn get_plugin_path(plugin_name: &str) -> String {
        format!("target\\{}\\coco_{}.dll", BUILD_TYPE, plugin_name)
    }
}

#[cfg(test)]
mod tests {
    use crate::plugin_manager::PluginManager;

    #[ignore]
    #[test]
    fn test_plugin_run_in_local() {
        let plugin = PluginManager::plugin("swagger");
        assert_eq!("coco.swagger", plugin.name());
    }
}
