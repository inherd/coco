#[macro_use]
extern crate lazy_static;
extern crate serde;

use core_model::CocoConfig;
use plugin_interface::PluginInterface;

use std::process::Command;

pub mod cmd_ctags;
pub mod coco_struct;
pub mod ctags_opt;
pub mod ctags_parser;
pub mod struct_analysis;

pub struct CocoStructAnalysis {}

impl PluginInterface for CocoStructAnalysis {
    fn name(&self) -> &'static str {
        "coco.struct_analysis"
    }

    fn on_plugin_load(&self) {
        match Command::new("ctags").spawn() {
            Ok(_) => {}
            Err(e) => {
                println!("`ctags` was not found! please install or follow link to setup: https://github.com/phodal/coco/blob/master/DEVELOPMENT.md#install-ctags");
                panic!("Error: {:?}", e);
            }
        };
    }

    fn on_plugin_unload(&self) {}

    fn execute(&self, config: CocoConfig) {
        struct_analysis::execute(config);
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

#[cfg(test)]
mod tests {
    use crate::coco_struct::ClassInfo;
    use crate::struct_analysis::execute;
    use core_model::{CocoConfig, RepoConfig};
    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;

    pub fn ctags_fixtures_dir() -> PathBuf {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();
        let ctags_dir = root_dir
            .clone()
            .join("_fixtures")
            .join("ctags")
            .join("source");

        return ctags_dir;
    }

    #[test]
    #[ignore]
    #[cfg(not(windows))]
    fn should_run_struct_analysis() {
        let mut repos = vec![];
        repos.push(RepoConfig {
            url: format!("{}", ctags_fixtures_dir().display()),
            languages: None,
        });
        let config = CocoConfig {
            repos: repos,
            plugins: None,
        };

        execute(config);

        let output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(".coco")
            .join("reporter")
            .join("struct_analysis")
            .join("source.json");

        let mut file = File::open(output_dir).unwrap();
        let mut code = String::new();
        file.read_to_string(&mut code).unwrap();
        let classes: Vec<ClassInfo> = serde_json::from_str(&code).unwrap();
        assert_eq!(6, classes.len());
    }
}
