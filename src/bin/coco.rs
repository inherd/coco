use std::{fs, thread};

use clap::{App, Arg};

use coco::app::git_analysis;
use coco::domain::config::{CocoConfig, RepoConfig};
use coco::infrastructure::name_format;
use coco::settings::Settings;

fn main() {
    let matches = App::new("Coco Program")
        .version("1.0")
        .author("Phodal <h@phodal.com>")
        .about("A effective DevOps analysis and auto-suggest tools.")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .get_matches();

    let config_file = matches.value_of("config").unwrap_or("coco.yml");
    let contents = fs::read_to_string(config_file).expect("reading config file error");
    let config: CocoConfig = serde_yaml::from_str(&contents).expect("parse config file error");

    println!("found config file: {}", config_file);

    let repo = config.repo.clone();
    run_analysis_repository(repo);
}

fn run_analysis_repository(repo: Vec<RepoConfig>) {
    let handle = thread::spawn(|| {
        for i in repo {
            let url_str = i.url.as_str();

            let results = git_analysis::get_repo(url_str);
            let file_name = name_format::from_url(url_str);

            let output_file = Settings::reporter_dir().join(file_name);

            fs::write(output_file, results).expect("cannot write file");
        }
    });

    handle.join().unwrap();
}
