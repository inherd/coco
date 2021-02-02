use std::{env, fs};

use clap::{App, Arg};
use rayon::prelude::*;

use coco::app::{architecture_analysis, git_analysis};
use coco::app::{cloc_analysis, framework_analysis};
use coco::domain::config::{CocoConfig, RepoConfig};
use coco::infrastructure::url_format;
use coco::settings::Settings;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone)]
pub struct CocoCliOption {
    pub branches: bool,
}

impl Default for CocoCliOption {
    fn default() -> Self {
        CocoCliOption { branches: false }
    }
}

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

    let config: CocoConfig;
    match fs::read_to_string(config_file) {
        Ok(content) => {
            config = serde_yaml::from_str(&content).expect("parse config file error");
        }
        Err(_) => {
            let mut repo = vec![];
            let current = env::current_dir().unwrap();
            repo.push(RepoConfig {
                url: current.into_os_string().to_str().unwrap().to_string(),
            });
            config = CocoConfig { repo }
        }
    }

    println!("found config file: {}", config_file);

    run_analysis(config.repo, cli_option);
}

fn run_analysis(repos: Vec<RepoConfig>, _cli_option: CocoCliOption) {
    // todo: add tasks for parallel run analysis tasks for one or more repos
    repos.par_iter().for_each(|repo| {
        let url_str = repo.url.as_str();

        // todo: thinking in refactor to patterns
        analysis_git(url_str);

        analysis_framework(url_str);
        analysis_cloc(url_str);
        analysis_architecture(url_str);
    });
}

fn analysis_framework(url_str: &str) {
    let path_buf = url_format::uri_to_path(url_str);
    let file_name = url_format::json_filename(url_str);

    let frameworks = framework_analysis::analysis(path_buf);
    let output_file = Settings::framework().join(file_name);

    fs::write(output_file, frameworks).expect("cannot write file");
}

fn analysis_git(url_str: &str) {
    let branches = git_analysis::analysis(url_str);
    let file_name = url_format::json_filename(url_str);

    let result = serde_json::to_string_pretty(&branches).unwrap();
    let output_file = Settings::git().join(file_name);

    fs::write(output_file, result).expect("cannot write file");
}

fn analysis_cloc(url_str: &str) {
    let path_buf = url_format::uri_to_path(url_str);
    let languages = cloc_analysis::analysis(path_buf);
    let file_name = url_format::json_filename(url_str);

    let result = serde_json::to_string_pretty(&languages).unwrap();
    let output_file = Settings::cloc().join(file_name);

    fs::write(output_file, result).expect("cannot write file");
}

fn analysis_architecture(url_str: &str) {
    let path_buf = url_format::uri_to_path(url_str);
    let branches_info = architecture_analysis::analysis(path_buf);
    let file_name = url_format::json_filename(url_str);

    let output_file = Settings::architecture().join(file_name);

    fs::write(output_file, branches_info).expect("cannot write file");
}
