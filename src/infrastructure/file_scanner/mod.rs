use std::path::PathBuf;

use walkdir::WalkDir;

use core_model::Settings;

pub fn search_git_projects(path: &PathBuf) -> Vec<String> {
    return search_projects(path, ".git");
}

pub fn lookup_projects() -> Vec<String> {
    let mut projects = vec![];
    // looking for one type
    let arch = Settings::architecture();
    for entry in WalkDir::new(&arch).max_depth(1) {
        let entry = entry.unwrap();
        let file_name = entry.file_name().to_os_string();
        if file_name.to_str().unwrap().contains(".json") {
            let file_name = file_name.to_str().unwrap();
            let project = file_name.replace(".json", "");
            projects.push(project);
        }
    }

    return projects;
}

pub fn search_projects(path: &PathBuf, filter: &str) -> Vec<String> {
    let mut results = vec![];
    let mut has_first_level = false;
    for entry in WalkDir::new(&path).max_depth(1) {
        let entry = entry.unwrap();
        if entry.path().ends_with(filter) {
            results.push(format!("{}", path.display()));
            has_first_level = true;
        }
    }

    if has_first_level {
        return results;
    }

    for entry in WalkDir::new(&path).max_depth(2) {
        let entry = entry.unwrap();
        if entry.path().ends_with(filter) {
            let strip_path = entry.path().strip_prefix(&path).unwrap();
            results.push(format!("{}", strip_path.display()));
        }
    }

    return results;
}

#[cfg(test)]
mod test {
    use std::path::{Path, PathBuf};

    use crate::infrastructure::file_scanner::search_projects;

    #[test]
    fn should_list_local_git() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let projects = search_projects(&path, ".git");

        assert_eq!(1, projects.len());
        assert_eq!(format!("{}", path.display()), projects[0]);
    }

    #[test]
    fn should_list_local_gittest() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("_fixtures")
            .join("repos")
            .join("root");
        let mut projects = search_projects(&path, ".gittest");

        assert_eq!(2, projects.len());
        projects.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

        let proj1 = Path::new("app1").join(".gittest");
        let proj2 = Path::new("app2").join(".gittest");

        assert_eq!(format!("{}", proj1.display()), projects[0]);
        assert_eq!(format!("{}", proj2.display()), projects[1]);
    }
}
