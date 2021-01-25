use std::{fs, thread};

use clap::{App, Arg};
use rayon::prelude::*;

use coco::app::{architecture_analysis, git_analysis};
use coco::app::{cloc_analysis, framework_analysis};
use coco::domain::config::{CocoConfig, RepoConfig};
use coco::infrastructure::url_format;
use coco::settings::Settings;

fn main() {
    let matches = App::new("Coco Program")
        .version("1.0")
        .author("Phodal <h@phodal.com>")
        .about("A DevOps Efficiency Analysis and Auto-suggestion Tool.")
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

    run_analysis(config.repo.clone());
}

fn run_analysis(repos: Vec<RepoConfig>) {
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
    let file_name = url_format::from(url_str);

    let frameworks = framework_analysis::analysis(path_buf);
    let output_file = Settings::reporter_dir(Some("framework")).join(file_name);

    fs::write(output_file, frameworks).expect("cannot write file");
}

fn analysis_git(url_str: &str) {
    let branches_info = git_analysis::branches_info(url_str);
    let file_name = url_format::from(url_str);

    let output_file = Settings::reporter_dir(Some("git")).join(file_name);

    fs::write(output_file, branches_info).expect("cannot write file");
}

fn analysis_cloc(url_str: &str) {
    let path_buf = url_format::uri_to_path(url_str);
    let languages = cloc_analysis::analysis(path_buf);
    let file_name = url_format::from(url_str);

    let result = serde_json::to_string_pretty(&languages).unwrap();
    let output_file = Settings::reporter_dir(Some("cloc")).join(file_name);

    fs::write(output_file, result).expect("cannot write file");
}

fn analysis_architecture(url_str: &str) {
    let path_buf = url_format::uri_to_path(url_str);
    let branches_info = architecture_analysis::analysis(path_buf);
    let file_name = url_format::from(url_str);

    let output_file = Settings::reporter_dir(Some("architecture")).join(file_name);

    fs::write(output_file, branches_info).expect("cannot write file");
}

#[cfg(test)]
mod test {}
