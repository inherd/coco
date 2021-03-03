use core_model::url_format::uri_to_path;
use core_model::{url_format, CocoConfig, Settings};
use ignore::Walk;

use std::path::PathBuf;
use std::{fs, str};
use structopt::StructOpt;

use crate::coco_struct::ClassInfo;
use crate::ctags_cmd::CmdCtags;
use crate::ctags_opt::Opt;
use crate::ctags_parser::CtagsParser;
use crate::plantuml_render::PlantUmlRender;

pub fn execute(config: CocoConfig) {
    for repo in &config.repos {
        let url_str = repo.url.as_str();

        let origin_files = files_from_path(url_str);
        let thread = count_thread(&origin_files);

        let mut opt = build_opt(thread);

        if let Some(langs) = &repo.languages {
            opt.languages = Some(langs.join(","));
        }

        if let Some(configs) = config.get_plugin_config("struct_analysis") {
            for config in &configs {
                if config.key == "ctags" {
                    opt.bin_ctags = PathBuf::from(config.value.clone());
                }
            }
        }

        let files = files_by_thread(origin_files, &opt);
        let classes = run_ctags(&opt, &files);

        let result = serde_json::to_string_pretty(&classes).unwrap();
        write_to_puml_file(url_str, &classes);
        write_to_json_file(url_str, &result);
    }
}

fn count_thread(origin_files: &Vec<String>) -> usize {
    let mut thread = origin_files.len();
    let default_ptags_thread = 8;
    if thread >= default_ptags_thread {
        thread = default_ptags_thread;
    }
    thread
}

fn write_to_json_file(url_str: &str, result: &String) {
    let file_name = url_format::json_filename(url_str);
    let output_file = Settings::struct_dir().join(file_name);
    fs::write(output_file, result).expect("cannot write file");
}

fn write_to_puml_file(url_str: &str, classes: &Vec<ClassInfo>) {
    let file_name = url_format::puml_filename(url_str);
    let output_file = Settings::struct_dir().join(file_name);
    let result = PlantUmlRender::render(classes);
    fs::write(output_file, result).expect("cannot write file");
}

fn run_ctags(opt: &Opt, files: &Vec<String>) -> Vec<ClassInfo> {
    let outputs = CmdCtags::call(&opt, &files).unwrap();
    let mut iters = Vec::new();
    for o in &outputs {
        let iter = if opt.validate_utf8 {
            str::from_utf8(&o.stdout).unwrap().lines()
        } else {
            unsafe { str::from_utf8_unchecked(&o.stdout).lines() }
        };
        iters.push(iter);
    }

    let parser = CtagsParser::parse_str(iters);
    let classes = parser.classes();

    classes
}

fn files_from_path(url_str: &str) -> Vec<String> {
    let mut origin_files = vec![];
    let path = uri_to_path(url_str);
    for result in Walk::new(path) {
        if let Ok(entry) = result {
            if entry.file_type().unwrap().is_file() {
                origin_files.push(format!("{}", entry.path().display()))
            }
        }
    }
    origin_files
}

fn files_by_thread(origin_files: Vec<String>, opt: &Opt) -> Vec<String> {
    let mut files = vec![String::from(""); opt.thread];
    for (i, f) in origin_files.iter().enumerate() {
        files[i % opt.thread].push_str(f);
        files[i % opt.thread].push_str("\n");
    }
    files
}

fn build_opt(thread: usize) -> Opt {
    let string = thread.to_string();
    let thread: &str = string.as_str();
    let args = vec!["ptags", "-t", thread, "--verbose=true", "--fields=+latinK"];
    let opt = Opt::from_iter(args.iter());
    opt
}
