use std::io::{Cursor, Read};
use std::path::Path;
use std::process::exit;
use std::{env, fs, io};

use crate::coco_error::CocoError;

pub struct PluginHelper;

impl PluginHelper {
    pub fn setup_plugins(plugins_path: &Path, version: &str) {
        PluginHelper::create_plugins_dir(plugins_path)
            .and_then(|msg| {
                println!("{}", msg);
                PluginHelper::download_plugins(version)
            })
            .and_then(|reader| PluginHelper::unzip_plugins(reader, plugins_path))
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

    fn download_plugins(version: &str) -> Result<Cursor<Vec<u8>>, CocoError> {
        let target = format!(
            "https://github.com/inherd/coco/releases/download/v{}/coco_plugins_{}.zip",
            version,
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
}

#[cfg(test)]
mod test {
    use crate::app::plugin_helper::PluginHelper;
    use std::path::Path;

    // todo: add in GitHub actions
    #[ignore]
    #[test]
    fn should_download_plugin() {
        let temp_path = Path::new("temp");
        PluginHelper::setup_plugins(temp_path, "0.5.0");
    }
}
