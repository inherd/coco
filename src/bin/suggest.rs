use clap::{App, Arg};
use coco::app::CocoOpt;
use core_model::CocoConfig;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("Coco")
        .version(VERSION)
        .author("Inherd Group")
        .about("A DevOps Efficiency Analysis and Auto-suggestion Tool.")
        .arg(
            Arg::with_name("concept")
                .long("concept")
                .value_name("FILE")
                .help("list concept of some words")
                .takes_value(true),
        )
        .get_matches();

    let config_file = matches.value_of("config").unwrap_or("coco.yml");

    let _cli_option = CocoOpt::default();
    let _config = CocoConfig::from_file(config_file);

    println!("found config file: {}", config_file);
}
