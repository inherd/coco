use std::{env, fs};

use clap::{App, Arg};
use rayon::prelude::*;

use coco::app::architecture_analysis;
use coco::app::cmd::CocoCliOption;
use coco::app::git_analysis::{branch_analysis, commit_analysis};
use coco::app::{cloc_analysis, framework_analysis};
use coco::domain::config::{CocoConfig, RepoConfig};
use coco::infrastructure::url_format;
use coco::settings::Settings;
use std::time::Instant;

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

    run_analysis(config.repo, cli_option);
}

fn create_config(config_file: &str) -> CocoConfig {
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
    config
}

fn run_analysis(repos: Vec<RepoConfig>, _cli_option: CocoCliOption) {
    // todo: add tasks for parallel run analysis tasks for one or more repos
    let start = Instant::now();
    repos.par_iter().for_each(|repo| {
        let url_str = repo.url.as_str();

        // todo: thinking in refactor to patterns
        analysis_branches(url_str);
        analysis_commits(url_str);

        analysis_framework(url_str);
        analysis_cloc(url_str);
        analysis_architecture(url_str);
    });

    println!("finish process in {:?}ms", start.elapsed().as_millis());
}

fn analysis_framework(url_str: &str) {
    let path_buf = url_format::uri_to_path(url_str);
    let file_name = url_format::json_filename(url_str);

    let frameworks = framework_analysis::analysis(path_buf);
    let output_file = Settings::framework().join(file_name);

    fs::write(output_file, frameworks).expect("cannot write file");
}

fn analysis_branches(url_str: &str) {
    let branches = branch_analysis::analysis(url_str);
    let file_name = url_format::json_filename(url_str);

    let result = serde_json::to_string_pretty(&branches).unwrap();
    let output_file = Settings::git().join(file_name);

    fs::write(output_file, result).expect("cannot write file");
}

fn analysis_commits(url_str: &str) {
    let branches = commit_analysis::analysis(url_str);
    let file_name = url_format::json_filename_suffix(url_str, Some("-commits"));

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
    }
}
