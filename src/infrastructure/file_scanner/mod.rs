use std::path::Path;
use walkdir::WalkDir;

pub fn find_git_projects<P: AsRef<Path>>(path: P) -> Vec<String> {
    let mut results = vec![];
    for entry in WalkDir::new(path).max_depth(1) {
        let entry = entry.unwrap();
        if entry.path().ends_with(".git") {
            println!("{}", entry.path().display());
            results.push(entry.path().display().to_string())
        }
    }

    return results;
}

#[cfg(test)]
mod test {
    use crate::infrastructure::file_scanner::find_git_projects;
    use std::path::PathBuf;

    #[test]
    fn should_list_local_git() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let projects = find_git_projects(path);
        assert_eq!(1, projects.len());
    }
}
