#[macro_use]
extern crate lazy_static;
extern crate serde;

use core_model::CocoConfig;
use core_model::PluginInterface;

use std::process::Command;

pub use ctags::ctags_cmd;
pub use ctags::ctags_opt;
pub use ctags::ctags_parser;

pub use plantuml::plantuml_render;

pub mod coco_struct;
pub mod coco_struct_plugin;
pub mod ctags;
pub mod plantuml;

pub struct CocoStructAnalysis {}

impl PluginInterface for CocoStructAnalysis {
    fn name(&self) -> &'static str {
        "coco.struct_analysis"
    }

    fn on_plugin_load(&self) {
        match Command::new("ctags").spawn() {
            Ok(_) => {}
            Err(e) => {
                show_ctags_install_help();
                panic!("Error: {:?}", e);
            }
        };
    }

    fn on_plugin_unload(&self) {}

    fn execute(&self, config: CocoConfig) {
        coco_struct_plugin::execute(config);
    }
}

#[cfg(target_os = "linux")]
fn show_ctags_install_help() {
    println!(
        "install ctags on Ubuntu:

sudo snap install universal-ctags
"
    );
}

#[cfg(target_os = "macos")]
fn show_ctags_install_help() {
    println!(
        "install ctags on macOS:

brew update
brew install --HEAD universal-ctags/universal-ctags/universal-ctags
"
    )
}

#[cfg(target_os = "windows")]
fn show_ctags_install_help() {
    println!(
        "install ctags on Windows:

choco install universal-ctags

or download from: https://github.com/universal-ctags/ctags-win32/releases
"
    );
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
    use crate::coco_struct_plugin::execute;
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

        let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(".coco")
            .join("reporter")
            .join("struct");
        let output_dir = base_dir.join("source.json");

        let mut file = File::open(output_dir).unwrap();
        let mut code = String::new();
        file.read_to_string(&mut code).unwrap();
        let classes: Vec<ClassInfo> = serde_json::from_str(&code).unwrap();
        assert_eq!(12, classes.len());

        let output_dir = base_dir.join("source.puml");
        let _file = File::open(output_dir).unwrap();
    }
}
