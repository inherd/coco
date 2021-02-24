use std::path::Path;

use crate::files::list_file_names;
use crate::jvm::psa_jvm::ModuleAnalyzer;
use crate::{Module, Project};

pub struct MavenModuleAnalyzer {}

impl ModuleAnalyzer for MavenModuleAnalyzer {
    fn analysis(&self, module_path: &str) -> Option<Module> {
        let module_name = get_module_name(module_path);
        match has_build_file(module_path) {
            true => Some(Module::new(module_name.as_str(), module_path)),
            _ => None,
        }
    }

    fn is_related(&self, project: &Project) -> bool {
        project.project_type == "maven"
    }
}

fn has_build_file(module_path: &str) -> bool {
    let file_names = list_file_names(module_path);
    for file_name in file_names.iter() {
        return match file_name.as_str() {
            "pom.xml" => true,
            _ => continue,
        };
    }
    false
}

fn get_module_name(project_path: &str) -> String {
    Path::new(project_path)
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap()
}
