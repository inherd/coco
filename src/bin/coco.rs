use std::fs::OpenOptions;
use std::{
    env,
    io::{Cursor, Read, Write},
    process::exit,
};
use std::{fs, io, path::Path};

use clap::{App, Arg};
use reqwest;
use zip;

use coco::app::analysis;
use coco::app::cmd::CocoCliOption;
use core_model::CocoConfig;
use plugin_manager::plugin_manager::PluginManager;

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
        .subcommand(
            App::new("init")
                .version(VERSION)
                .author("Inherd Group")
                .about("A DevOps Efficiency Analysis and Auto-suggestion Tool."),
        )
        .subcommand(
            App::new("plugins")
                .version(VERSION)
                .author("Inherd Group")
                .about("A DevOps Efficiency Analysis and Auto-suggestion Tool."),
        )
        .get_matches();

    if matches.is_present("init") {
        create_config_file();
        exit(0);
    }

    if matches.is_present("plugins") {
        setup_plugins();
        exit(0);
    }

    let config_file = matches.value_of("config").unwrap_or("coco.yml");

    let cli_option = CocoCliOption::default();

    let config = CocoConfig::from_file(config_file);

    println!("found config file: {}", config_file);

    let analyst = analysis::Analyst::from(&config);
    analyst.analysis(cli_option);

    if config.plugins.is_some() {
        run_plugins(&config);
    }
}

fn run_plugins(config: &CocoConfig) {
    for plugin in config.plugins.as_ref().unwrap().iter() {
        PluginManager::run(&plugin.name, config.clone());
    }
}

fn create_config_file() {
    println!("creating coco.yml");
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open("coco.yml")
        .map(|mut file| file.write(&serde_yaml::to_vec(&CocoConfig::default()).unwrap()))
    {
        Ok(_) => println!("success created"),
        Err(e) => println!("coco.yml create faild: {}", e),
    }
}

fn setup_plugins() {
    let plugins_path = Path::new("coco_plugins");
    create_plugin_dir(&plugins_path);

    let reader = download_plugins();

    unzip_plugins(reader, plugins_path)
}

fn create_plugin_dir(path_name: &Path) {
    if path_name.exists() {
        return ();
    }
    match fs::create_dir(&path_name) {
        Ok(_) => println!("create plugin dir success"),
        Err(e) => {
            eprintln!("create plugin dir failed: {}", e);
            exit(1);
        }
    }
}

fn download_plugins() -> Cursor<Vec<u8>> {
    let target = format!(
        "https://github.com/inherd/coco/releases/download/v{}/coco_plugins_{}.zip",
        VERSION,
        env::consts::OS
    );
    println!("download from {}", target);

    let mut response = reqwest::blocking::get(&target).expect("download error");
    let mut buf: Vec<u8> = vec![];
    response.read_to_end(&mut buf).unwrap();
    Cursor::new(buf)
}

fn unzip_plugins(reader: Cursor<Vec<u8>>, plugins_path: &Path) {
    let mut archive = zip::ZipArchive::new(reader).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let out_path = match file.enclosed_name() {
            Some(path) => plugins_path.join(path),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("Plugin {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, out_path.display());
            fs::create_dir_all(&out_path).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                out_path.display(),
                file.size()
            );
            if let Some(p) = out_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&out_path).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&out_path, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use core_model::CocoConfig;
    use std::env;

    #[test]
    fn should_set_default_config() {
        let config = CocoConfig::from_file("");
        let current = env::current_dir().unwrap();
        let url = current.into_os_string().to_str().unwrap().to_string();

        assert_eq!(config.repos.len(), 1);
        assert_eq!(url, config.repos[0].url);
        assert!(config.plugins.is_none())
    }
}
