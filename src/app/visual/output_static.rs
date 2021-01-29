use rust_embed::RustEmbed;
use std::fs;
use std::path::PathBuf;

#[derive(RustEmbed)]
#[folder = "web/"]
struct Asset;

pub fn run(path_str: &str) {
    let path = PathBuf::from(path_str);

    for file in Asset::iter() {
        let file_name = format!("{}", file.as_ref());
        let file_path = &path.join(file.as_ref());

        let content = Asset::get(&file_name).unwrap();
        let _ = fs::create_dir_all(&file_path.parent().unwrap());

        println!("write to file: {}", file_path.display());
        fs::write(file_path, content).expect("cannot write file");
    }
}
