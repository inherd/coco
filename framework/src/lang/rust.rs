use walkdir::DirEntry;

pub fn get_tag<'a>(entry: &DirEntry) -> Option<&'a str> {
    let file_name = entry.file_name().to_str().unwrap();
    match file_name {
        "Cargo.toml" => Some("workspace.cargo"),
        _ => None,
    }
}
