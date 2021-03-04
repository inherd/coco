use std::path::PathBuf;

use dlopen::wrapper::{Container, WrapperApi};

use core_model::CocoConfig;
use core_model::PluginInterface;

const BUILD_TYPE: &str = if cfg!(debug_assertions) {
    "debug"
} else {
    "release"
};

#[derive(WrapperApi)]
struct Wrapper {
    plugin: fn() -> Box<dyn PluginInterface>,
}

pub struct PluginManager {
    config: CocoConfig,
}

impl PluginManager {
    pub fn run(&self, plugin_name: &str) {
        let root = PathBuf::from(".");
        let production_plugins = root.join("coco_plugins");

        let path;
        if production_plugins.exists() {
            let plugin_path = Self::get_plugin_path(plugin_name, true);
            path = root.join(plugin_path);
        } else {
            let debug_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let plugin_path = Self::get_plugin_path(plugin_name, false);
            // because cargo project settings
            path = debug_root.parent().unwrap().join(plugin_path);
        }

        println!("search plugins in path: {:?}", path.display());

        let cont: Container<Wrapper> =
            unsafe { Container::load(path) }.expect("Could not open library or load symbols");

        let plugin = cont.plugin();

        // todo: return plugin interface will cause crash in Ubuntu.
        plugin.on_plugin_load();
        plugin.execute(self.config.clone());
    }

    pub fn run_all(&self) {
        if self.config.plugins.is_none() {
            return ();
        }
        for plugin in self.config.plugins.as_ref().unwrap().iter() {
            self.run(&plugin.name);
        }
    }

    #[cfg(target_os = "linux")]
    fn get_plugin_path(plugin_name: &str, for_production: bool) -> String {
        if for_production {
            format!("coco_plugins/libcoco_{}.so", plugin_name)
        } else {
            format!("target/{}/libcoco_{}.so", BUILD_TYPE, plugin_name)
        }
    }

    #[cfg(target_os = "macos")]
    fn get_plugin_path(plugin_name: &str, for_production: bool) -> String {
        if for_production {
            format!("coco_plugins/libcoco_{}.dylib", plugin_name)
        } else {
            format!("target/{}/libcoco_{}.dylib", BUILD_TYPE, plugin_name)
        }
    }

    #[cfg(target_os = "windows")]
    fn get_plugin_path(plugin_name: &str, for_production: bool) -> String {
        if for_production {
            format!("coco_plugins\\coco_{}.dll", plugin_name)
        } else {
            format!("target\\{}\\coco_{}.dll", BUILD_TYPE, plugin_name)
        }
    }
}

impl From<&CocoConfig> for PluginManager {
    fn from(config: &CocoConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plugin_manager::PluginManager;
    use core_model::coco_config::CocoPlugin;
    use core_model::{CocoConfig, RepoConfig};

    #[test]
    fn test_plugin_run_all_in_local() {
        let mut config = CocoConfig::default();
        let plugins_str = vec!["swagger", "struct", "pipeline", "container"];
        let mut plugins = vec![];
        for plugin in plugins_str {
            plugins.push(CocoPlugin {
                name: plugin.to_string(),
                config: None,
            });
        }

        config.plugins = Some(plugins);

        let manager = PluginManager::from(&config);
        manager.run_all();
    }

    #[test]
    fn test_single_run_method() {
        let config = CocoConfig {
            repos: vec![RepoConfig::default()],
            plugins: Some(vec![CocoPlugin {
                name: "swagger".to_string(),
                config: None,
            }]),
        };
        let manager = PluginManager::from(&config);
        manager.run("swagger");
    }

    #[test]
    fn test_manager_from_config() {
        let config = CocoConfig::default();
        let manager = PluginManager::from(&config);
        assert_eq!(manager.config, config);
    }
}
