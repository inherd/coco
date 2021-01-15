use clap::{App, Arg};
use coco::app::git_analysis::get_repo;
use coco::domain::config::{CocoConfig, RepoConfig};
use coco::infrastructure::name_format;
use std::{fs, thread};
use std::path::Path;

fn main() {
    let matches = App::new("Coco Program")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
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

    let repo = config.repo.clone();
    run_analysis_repository(repo);
}

fn run_analysis_repository(repo: Vec<RepoConfig>) {
    let handle = thread::spawn(|| {
        for x in repo {
            let results = get_repo(x.url.as_str());
            let file_name = name_format::from_url(x.url.as_str());

            let root = Path::new(".coco");
            let reporter_buf = root.join("reporter");
            let _ = fs::create_dir_all(reporter_buf.clone());

            let output_file = reporter_buf.join(file_name);

            fs::write(output_file, results).expect("cannot write file");
        }
    });

    handle.join().unwrap();
}
