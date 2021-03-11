use core_model::CocoConfig;
use core_model::PluginInterface;

pub mod coco_container_plugin;

pub struct CocoContainer {}

impl PluginInterface for CocoContainer {
    fn name(&self) -> &'static str {
        "coco.swagger"
    }

    fn on_plugin_load(&self) {}

    fn on_plugin_unload(&self) {}

    fn execute(&self, config: CocoConfig) {
        println!("{:?}", config);
    }
}

impl Default for CocoContainer {
    fn default() -> Self {
        CocoContainer {}
    }
}

#[no_mangle]
pub fn plugin() -> Box<dyn PluginInterface> {
    Box::new(CocoContainer::default())
}

#[cfg(test)]
mod tests {
    use crate::coco_container_plugin::analysis;
    use dockerfile_parser::Dockerfile;
    use std::path::PathBuf;

    pub fn dockerfile_dir() -> PathBuf {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();

        return root_dir.clone().join("_fixtures").join("dockerfile");
    }

    #[test]
    pub fn should_parse_dockerfile() {
        let dockerfile = analysis(&dockerfile_dir().join("Go.Dockerfile"));

        for stage in dockerfile.iter_stages() {
            println!("stage #{}", stage.index);
            for ins in stage.instructions {
                println!("  {:?}", ins);
            }
        }
    }
}
