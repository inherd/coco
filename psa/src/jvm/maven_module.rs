use std::path::Path;

use crate::files::{list_file_names, list_sub_dirs};
use crate::jvm::psa_jvm::ModuleAnalyzer;
use crate::{Module, Project};

pub struct MavenModuleAnalyzer {}

impl MavenModuleAnalyzer {
    fn analysis_sub_modules(&self, module_path: &str) -> Vec<Module> {
        let mut sub_modules = Vec::new();
        let sub_dirs = list_sub_dirs(Path::new(module_path));
        for each_sub_dir in sub_dirs.iter() {
            let sub_module = self.analysis(each_sub_dir);
            match sub_module {
                Some(sub_module) => sub_modules.push(sub_module),
                _ => continue,
            }
        }
        sub_modules
    }
}

impl ModuleAnalyzer for MavenModuleAnalyzer {
    fn analysis(&self, module_path: &str) -> Option<Module> {
        let mut module = create_module(module_path);
        if !module.is_none() {
            let sub_modules = &mut self.analysis_sub_modules(module_path);
            module.as_mut().unwrap().add_sub_modules(sub_modules);
        }
        module
    }

    fn is_related(&self, project: &Project) -> bool {
        project.project_type == "maven"
    }
}

fn create_module(module_path: &str) -> Option<Module> {
    let module_name = get_module_name(module_path);
    match has_build_file(module_path) {
        true => Some(Module::new(module_name.as_str(), module_path)),
        _ => None,
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
