use pathdiff::diff_paths;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn list_file_names<P: AsRef<Path>>(path: P) -> Vec<String> {
    let mut files = Vec::new();
    let walk_dir = WalkDir::new(path);
    for dir_entry in walk_dir.max_depth(1).into_iter() {
        if dir_entry.is_err() {
            panic!("{}", dir_entry.err().unwrap());
        }

        let entry = dir_entry.unwrap();
        if entry.metadata().unwrap().is_file() {
            files.push(entry.file_name().to_os_string().into_string().unwrap());
        }
    }
    files
}

pub fn list_all<P: AsRef<Path>>(path: P) -> HashSet<String> {
    let mut dirs = HashSet::new();
    let walk_dir = WalkDir::new(path);
    for dir_entry in walk_dir
        .min_depth(1)
        .sort_by(|a, b| a.file_name().cmp(b.file_name()))
        .into_iter()
    {
        if dir_entry.is_err() {
            panic!("{}", dir_entry.err().unwrap());
        }

        dirs.insert(dir_entry.unwrap().path().display().to_string());
    }
    dirs
}

pub fn list_sub_dirs<P: AsRef<Path>>(path: P) -> Vec<String> {
    let mut dirs = Vec::new();
    let walk_dir = WalkDir::new(path);
    for dir_entry in walk_dir
        .min_depth(1)
        .max_depth(1)
        .sort_by(|a, b| a.file_name().cmp(b.file_name()))
        .into_iter()
    {
        if dir_entry.is_err() {
            panic!("{}", dir_entry.err().unwrap());
        }

        let entry = dir_entry.unwrap();
        if entry.metadata().unwrap().is_dir() {
            dirs.push(entry.path().display().to_string())
        }
    }
    dirs
}

pub fn find_in_path(root_path: &str, file: Vec<&str>) -> Option<String> {
    let all_files = list_all(root_path);
    let mut parent_path = PathBuf::from(root_path).to_path_buf();
    for each_part in file.into_iter() {
        parent_path.push(each_part);
    }
    match all_files.contains(&parent_path.display().to_string()) {
        true => Some(parent_path.display().to_string()),
        _ => None,
    }
}

pub fn join_path(root_path: &str, file: Vec<&str>) -> String {
    let mut parent_path = PathBuf::from(root_path).to_path_buf();
    for each_part in file.into_iter() {
        parent_path.push(each_part);
    }
    parent_path.display().to_string()
}

pub fn to_relative_path(base_path: &str, absolute_path: &str) -> String {
    diff_paths(absolute_path, base_path)
        .unwrap()
        .display()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::files::to_relative_path;

    #[test]
    fn should_convert_absolute_path_to_relative_path() {
        let relative_path = to_relative_path("/a/b/c", "/a/b/c/d/");

        assert_eq!(relative_path, "d");
    }
}
