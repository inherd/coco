use rust_embed::RustEmbed;
use std::fs;
use std::path::Path;

#[derive(RustEmbed)]
#[folder = "web/"]
struct Asset;

pub fn run<P: AsRef<Path>>(path: P) {
    copy_reporter_to(&path.as_ref());

    for file in Asset::iter() {
        let file_name = format!("{}", file.as_ref());
        let file_path = &path.as_ref().join(file.as_ref());

        let content = Asset::get(&file_name).unwrap();
        let _ = fs::create_dir_all(&file_path.parent().unwrap());

        println!("write to file: {}", file_path.display());
        fs::write(file_path, content).expect("cannot write file");
    }
}

pub fn copy_reporter_to(_path: &Path) {
    // todo: make reporter copy
}
