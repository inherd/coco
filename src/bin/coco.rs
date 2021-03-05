use std::fs::OpenOptions;
use std::{
    env, fs, io,
    io::{Cursor, Read},
    path::Path,
    process::exit,
};

use reqwest;
use structopt::StructOpt;
use zip;

use coco::app::analysis;
use coco::app::coco_opt::CocoCommand;
use coco::app::CocoOpt;
use coco::coco_error::CocoError;
use core_model::CocoConfig;
use plugin_manager::plugin_manager::PluginManager;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let opt: CocoOpt = CocoOpt::from_args();
    if let Some(sub_cmd) = opt.cmd {
        match sub_cmd {
            CocoCommand::Init => {
                create_config_file();
                exit(0);
            }
            CocoCommand::Plugins => {
                setup_plugins();
                exit(0);
            }
        }
    }

    let config_file = &opt.config_file;
    let config = CocoConfig::from_file(config_file);

    println!("found config file: {}", config_file);
    if opt.debug {
        println!("{:?}", opt);
    }

    let analyst = analysis::Analyst::from(&config);
    analyst.analysis(opt);

    let plugin_manager = PluginManager::from(&config);
    plugin_manager.run_all();
}

fn create_config_file() {
    println!("creating coco.yml");
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open("coco.yml")
        .map(|file| serde_yaml::to_writer(file, &CocoConfig::default()).unwrap())
    {
        Ok(_) => println!("success created"),
        Err(e) => println!("coco.yml create failed: {}", e),
    }
}

fn setup_plugins() {
    let plugins_path = Path::new("coco_plugins");
    create_plugins_dir(&plugins_path)
        .and_then(|msg| {
            println!("{}", msg);
            download_plugins()
        })
        .and_then(|reader| unzip_plugins(reader, plugins_path))
        .unwrap_or_else(|err_msg| {
            println!("Failed: {}", err_msg);
            exit(1);
        });
}

fn create_plugins_dir(path_name: &Path) -> Result<&'static str, CocoError> {
    if path_name.exists() {
        return Ok("plugins dir already exists");
    }
    fs::create_dir(&path_name)
        .and_then(|_| Ok("create plugins dir success"))
        .or(Err(CocoError::new("created failed")))
}

fn download_plugins() -> Result<Cursor<Vec<u8>>, CocoError> {
    let target = format!(
        "https://github.com/inherd/coco/releases/download/v{}/coco_plugins_{}.zip",
        VERSION,
        env::consts::OS
    );
    println!("download from {}", target);

    let mut response = reqwest::blocking::get(&target)?;
    let mut buf: Vec<u8> = vec![];
    response.read_to_end(&mut buf)?;
    Ok(Cursor::new(buf))
}

fn unzip_plugins(reader: Cursor<Vec<u8>>, plugins_path: &Path) -> Result<(), CocoError> {
    let mut archive = zip::ZipArchive::new(reader)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
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
            fs::create_dir_all(&out_path)?;
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                out_path.display(),
                file.size()
            );
            if let Some(p) = out_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = fs::File::create(&out_path)?;
            io::copy(&mut file, &mut outfile)?;
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&out_path, fs::Permissions::from_mode(mode))?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::env;

    use core_model::CocoConfig;

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
