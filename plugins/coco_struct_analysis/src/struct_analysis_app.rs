use core_model::url_format::uri_to_path;
use core_model::{url_format, CocoConfig, Settings};
use ignore::Walk;

use std::{fs, str};
use structopt::StructOpt;

use crate::cmd_ctags::CmdCtags;
use crate::ctags_opt::Opt;
use crate::ctags_parser::CtagsParser;

pub fn execute_struct_analysis(config: CocoConfig) {
    for repo in config.repo {
        let mut origin_files = vec![];
        let url_str = repo.url.as_str();
        let path = uri_to_path(url_str);
        for result in Walk::new(path) {
            if let Ok(entry) = result {
                if entry.file_type().unwrap().is_file() {
                    origin_files.push(format!("{}", entry.path().display()))
                }
            }
        }
        let mut thread = origin_files.len();
        if thread >= 8 {
            thread = 8;
        }
        let opt = build_opt(thread);

        let mut files = vec![String::from(""); opt.thread];
        for (i, f) in origin_files.iter().enumerate() {
            files[i % opt.thread].push_str(f);
            files[i % opt.thread].push_str("\n");
        }

        let outputs = CmdCtags::call(&opt, &files).unwrap();
        let out_str = str::from_utf8(&outputs[0].stdout).unwrap();

        let parser = CtagsParser::parse_str(out_str);
        let classes = parser.classes();

        let file_name = url_format::json_filename(url_str);
        let output_file = Settings::struct_analysis().join(file_name);

        let result = serde_json::to_string_pretty(&classes).unwrap();
        fs::write(output_file, result).expect("cannot write file");
    }
}

fn build_opt(thread: usize) -> Opt {
    let string = thread.to_string();
    let thread: &str = string.as_str();
    let args = vec![
        "ptags",
        "-t",
        thread,
        // "--bin-ctags=/usr/local/bin/ctags",
        "--verbose=true",
        "--fields=+latinK",
    ];
    let opt = Opt::from_iter(args.iter());
    opt
}
