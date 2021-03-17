use std::fs::OpenOptions;
use std::{env, path::Path, process::exit};

use structopt::StructOpt;

use coco::app::analysis;
use coco::app::plugin_helper::PluginHelper;
use coco::app::PluginManager;
use coco::domain::{CocoCommand, CocoOpt};
use core_model::CocoConfig;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let opt: CocoOpt = CocoOpt::from_args();
    if let Some(sub_cmd) = opt.cmd {
        match sub_cmd {
            CocoCommand::Init => {
                create_config_file();
                exit(0);
            }
            CocoCommand::Plugins => {
                let plugins_path = Path::new("coco_plugins");
                PluginHelper::setup(&plugins_path, VERSION);
                exit(0);
            }
        }
    }

    let config_file = &opt.config_file;
    let config = CocoConfig::from_file(config_file);

    let is_debug = opt.debug.clone();
    if is_debug {
        println!("found config file: {}", config_file);
        println!("{:?}", opt);
    }

    let analyst = analysis::Analyst::from(&config);
    analyst.analysis(opt);

    let plugin_manager = PluginManager::from(&config);
    plugin_manager.run_all(is_debug);
}

fn create_config_file() {
    println!("creating coco.yml");
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open("coco.yml")
        .map(|file| serde_yaml::to_writer(file, &CocoConfig::default()).unwrap())
    {
        Ok(_) => println!("success created"),
        Err(e) => println!("coco.yml create failed: {}", e),
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use core_model::CocoConfig;

    #[test]
    fn should_set_default_config() {
        let config = CocoConfig::from_file("");
        let current = env::current_dir().unwrap();
        let url = current.into_os_string().to_str().unwrap().to_string();

        assert_eq!(config.repos.len(), 1);
        assert_eq!(url, config.repos[0].url);
        assert!(config.plugins.is_none())
    }
}
