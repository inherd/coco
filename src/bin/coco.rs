use std::{env, fs};

use clap::{App, Arg};

use coco::app::analysis;
use coco::app::cmd::CocoCliOption;
use core_model::{CocoConfig, RepoConfig};
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
        .get_matches();

    let config_file = matches.value_of("config").unwrap_or("coco.yml");

    let cli_option = CocoCliOption::default();

    let config = create_config(config_file);

    println!("found config file: {}", config_file);

    run_plugins(&config);
    let analyst = analysis::Analyst::from(&config);
    analyst.analysis(cli_option);
}

fn create_config(config_file: &str) -> CocoConfig {
    match fs::read_to_string(config_file) {
        Ok(content) => serde_yaml::from_str(&content).expect("parse config file error"),
        Err(_) => {
            let mut repo = vec![];
            let current = env::current_dir().unwrap();
            repo.push(RepoConfig {
                url: current.into_os_string().to_str().unwrap().to_string(),
            });
            CocoConfig {
                repo,
                plugins: vec![],
            }
        }
    }
}

fn run_plugins(config: &CocoConfig) {
    for plugin in config.plugins.iter() {
        PluginManager::run(&plugin, config.clone());
    }
}

#[cfg(test)]
mod test {
    use crate::create_config;
    use std::env;

    #[test]
    fn should_set_default_config() {
        let config = create_config("");
        let current = env::current_dir().unwrap();
        let url = current.into_os_string().to_str().unwrap().to_string();

        assert_eq!(config.repo.len(), 1);
        assert_eq!(url, config.repo[0].url);
        assert_eq!(config.plugins.len(), 0);
    }
}
