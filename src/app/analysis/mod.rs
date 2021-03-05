mod architecture_analysis;
mod cloc_analysis;
mod framework_analysis;
mod git_analysis;
use git_analysis::*;
use std::fs;

use super::CocoOpt;
use core_model::url_format;
use core_model::Settings;
use core_model::{CocoConfig, RepoConfig};

use rayon::prelude::*;
use std::time::Instant;

pub struct Analyst {
    repos: Vec<RepoConfig>,
}

impl From<&CocoConfig> for Analyst {
    fn from(config: &CocoConfig) -> Self {
        Self {
            repos: config.repos.clone(),
        }
    }
}

impl Analyst {
    pub fn analysis(&self, cli_option: CocoOpt) {
        // todo: add tasks for parallel run analysis tasks for one or more repos
        let start = Instant::now();
        self.repos.par_iter().for_each(|repo| {
            let url_str = repo.url.as_str();
            // todo: thinking in refactor to patterns

            // todo: merge to one app?

            if cli_option.branches {
                analysis_branches(url_str);
            }
            if cli_option.commits {
                analysis_commits(url_str);
            }
            if cli_option.tags {
                analysis_tags(url_str);
            }
            if cli_option.file_history {
                analysis_file_history(url_str, cli_option.git_years.clone());
            }

            analysis_framework(url_str);
            analysis_cloc(url_str);
            analysis_architecture(url_str);
        });

        println!("finish process in {:?}ms", start.elapsed().as_millis());
    }
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

fn analysis_tags(url_str: &str) {
    let branches = tag_analysis::analysis(url_str);
    let file_name = url_format::json_filename_suffix(url_str, Some("-tags"));

    let result = serde_json::to_string_pretty(&branches).unwrap();
    let output_file = Settings::git().join(file_name);

    fs::write(output_file, result).expect("cannot write file");
}

fn analysis_file_history(url_str: &str, git_years: f64) {
    let tree = file_analysis::analysis(url_str, git_years);
    let file_name = url_format::json_filename_suffix(url_str, Some("-file-history"));

    let result = serde_json::to_string_pretty(&tree).unwrap();
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
