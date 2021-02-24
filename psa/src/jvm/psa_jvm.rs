use crate::project_structure_analyzer::StructureAnalyzer;
use crate::psa_project::Project;
use std::path::Path;
use walkdir::WalkDir;

pub struct JvmProjectStructureAnalyzer {}

impl Default for JvmProjectStructureAnalyzer {
    fn default() -> Self {
        JvmProjectStructureAnalyzer {}
    }
}

impl StructureAnalyzer for JvmProjectStructureAnalyzer {
    fn analysis(&self, project_path: &str) -> Project {
        let project_name = get_project_name(project_path);
        Project::new(project_name.as_str(), project_path)
    }

    fn is_related(&self, project_path: &str) -> bool {
        let files = list_file_names(project_path);
        for file_name in files.iter() {
            if is_build_file(file_name) {
                return true;
            }
        }
        false
    }
}

fn get_project_name(project_path: &str) -> String {
    Path::new(project_path)
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap()
}

fn is_build_file(file_name: &str) -> bool {
    match file_name {
        "build.gradle" => true,
        "pom.xml" => true,
        _ => false,
    }
}

fn list_file_names<P: AsRef<Path>>(path: P) -> Vec<String> {
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
