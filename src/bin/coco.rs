use std::{fs, thread};

use clap::{App, Arg};

use coco::app::git_analysis;
use coco::domain::config::{CocoConfig, RepoConfig};
use coco::infrastructure::url_format;
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

    run_analysis_repositories(config.repo.clone());
}

fn run_analysis_repositories(repos: Vec<RepoConfig>) {
    thread::spawn(|| {
        for repo in repos {
            // todo: add other analysis code in here
            analysis_repo(repo);
        }
    })
    .join()
    .unwrap();
}

fn analysis_repo(repo: RepoConfig) {
    let url_str = repo.url.as_str();

    let branches_info = git_analysis::get_repo(url_str);
    let file_name = url_format::from(url_str);

    let output_file = Settings::reporter_dir().join(file_name);

    fs::write(output_file, branches_info).expect("cannot write file");
}
