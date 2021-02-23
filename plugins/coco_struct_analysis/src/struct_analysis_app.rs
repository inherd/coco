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
        let mut files = vec![];

        let url_str = repo.url.as_str();
        let path = uri_to_path(url_str);
        for result in Walk::new(path) {
            if let Ok(entry) = result {
                if entry.file_type().unwrap().is_file() {
                    files.push(format!(
                        "{}",
                        fs::canonicalize(entry.path()).unwrap().display()
                    ))
                }
            }
        }

        let mut thread = files.len();
        if thread >= 8 {
            thread = 8;
        }
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

        let outputs = CmdCtags::call(&opt, &files).unwrap();
        let out_str = str::from_utf8(&outputs[0].stdout).unwrap();

        println!("{}", out_str);

        let parser = CtagsParser::parse_str(out_str);
        let classes = parser.classes();

        let file_name = url_format::json_filename(url_str);
        let output_file = Settings::struct_analysis().join(file_name);

        let result = serde_json::to_string_pretty(&classes).unwrap();
        fs::write(output_file, result).expect("cannot write file");
    }
}
