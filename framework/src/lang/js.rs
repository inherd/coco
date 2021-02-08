use walkdir::DirEntry;

pub fn get_tag<'a>(entry: &DirEntry) -> Option<&'a str> {
    let file_name = entry.file_name().to_str().unwrap();
    match file_name {
        "bower.json" | "bower_components" => Some("workspace.bower"),
        "package.json" | "node_modules" => Some("workspace.npm"),
        _ => None,
    }
}
