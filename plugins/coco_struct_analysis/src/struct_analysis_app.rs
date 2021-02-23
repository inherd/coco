use std::{fs, str};
use structopt::StructOpt;
use walkdir::{DirEntry, WalkDir};

use core_model::url_format::uri_to_path;
use core_model::{url_format, CocoConfig, Settings};

use crate::cmd_ctags::CmdCtags;
use crate::ctags_opt::Opt;
use crate::ctags_parser::CtagsParser;

pub fn execute_struct_analysis(config: CocoConfig) {
    let args = vec![
        "ptags",
        "-t",
        "1",
        // "--bin-ctags=/usr/local/bin/ctags",
        "--verbose=true",
        "--fields=+latinK",
    ];

    for repo in config.repo {
        let mut files = vec![];

        let url_str = repo.url.as_str();
        let path = uri_to_path(url_str);
        for entry in WalkDir::new(path) {
            let entry: DirEntry = entry.unwrap();
            if entry.file_type().is_file() {
                files.push(format!("{}", entry.path().display()));
            }
        }

        let opt = Opt::from_iter(args.iter());

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
