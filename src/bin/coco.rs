use std::fs::OpenOptions;
use std::{env, io::Write};

use clap::{App, Arg};

use coco::app::analysis;
use coco::app::cmd::CocoCliOption;
use core_model::CocoConfig;
use plugin_manager::plugin_manager::PluginManager;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("Coco")
        .version(VERSION)
        .author("Inherd Group")
        .about("A DevOps Efficiency Analysis and Auto-suggestion Tool.")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("config file")
                .takes_value(true),
        )
        .subcommand(
            App::new("init")
                .version(VERSION)
                .author("Inherd Group")
                .about("A DevOps Efficiency Analysis and Auto-suggestion Tool."),
        )
        .get_matches();

    if matches.is_present("init") {
        println!("creating coco.yml");
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open("coco.yml")
            .map(|mut file| file.write(&serde_yaml::to_vec(&CocoConfig::default()).unwrap()))
        {
            Ok(_) => println!("success created"),
            Err(_) => println!("coco.yml already exists"),
        }
        std::process::exit(0);
    }

    let config_file = matches.value_of("config").unwrap_or("coco.yml");

    let cli_option = CocoCliOption::default();

    let config = CocoConfig::from_file(config_file);

    println!("found config file: {}", config_file);

    if config.plugins.is_some() {
        run_plugins(&config);
    }

    let analyst = analysis::Analyst::from(&config);
    analyst.analysis(cli_option);
}

fn run_plugins(config: &CocoConfig) {
    for plugin in config.plugins.as_ref().unwrap().iter() {
        PluginManager::run(&plugin.name, config.clone());
    }
}

#[cfg(test)]
mod test {
    use core_model::CocoConfig;
    use std::env;

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
