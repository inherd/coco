use crate::coco_pipeline::CocoPipeline;
use core_model::url_format::uri_to_path;
use core_model::{url_format, CocoConfig, Settings};
use ignore::Walk;
use jenkinsfile::Jenkinsfile;
use std::fs;
use std::path::PathBuf;

pub fn execute(config: CocoConfig) {
    for repo in &config.repos {
        let url_str = repo.url.as_str();
        let origin_files = lookup_jenkinsfile(url_str);
        let mut results = vec![];

        for path in origin_files {
            let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
            if let Some(jenkinsfile) = Jenkinsfile::from_str(contents.as_str()) {
                results.push(CocoPipeline::from(jenkinsfile));
            }
        }

        let result = serde_json::to_string_pretty(&results).unwrap();
        write_to_json_file(url_str, &result);
    }
}

fn write_to_json_file(url_str: &str, result: &String) {
    let file_name = url_format::json_filename(url_str);
    let output_file = Settings::pipeline().join(file_name);
    fs::write(output_file, result).expect("cannot write file");
}

fn lookup_jenkinsfile(url_str: &str) -> Vec<PathBuf> {
    let path = uri_to_path(url_str);
    let mut pipeline_files = vec![];
    for result in Walk::new(path) {
        if let Ok(entry) = result {
            if !entry.file_type().unwrap().is_file() {
                continue;
            }

            if entry.file_name().to_str().unwrap() == "Jenkinsfile" {
                pipeline_files.push(entry.into_path());
            }
        }
    }

    pipeline_files
}
