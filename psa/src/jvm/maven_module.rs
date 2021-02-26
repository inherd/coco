use std::path::Path;

use crate::files::{find_in_path, list_file_names, list_sub_dirs};
use crate::jvm::psa_jvm::ModuleAnalyzer;
use crate::{Module, Project};

pub struct MavenModuleAnalyzer {}

impl MavenModuleAnalyzer {
    fn detect_sub_modules(&self, module_path: &str, module: &mut Option<Module>) {
        let sub_modules = &mut self.analysis_sub_modules(module_path);
        module.as_mut().unwrap().add_sub_modules(sub_modules);
        self.detect_content_root(module_path, module);
    }

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

    fn detect_content_root(&self, module_path: &str, mut module: &mut Option<Module>) {
        self.detect_source_root(module_path, &mut module);
        self.detect_resource_root(module_path, &mut module);
        self.detect_test_source_root(module_path, &mut module);
        self.detect_test_resource_root(module_path, &mut module);
    }

    fn detect_source_root(&self, module_path: &str, module: &mut Option<Module>) {
        let path = module_path;
        let source_root = find_in_path(path, vec!["src", "main", "java"]);
        match source_root {
            Some(source_root) => module.as_mut().unwrap().add_source_root(source_root),
            _ => (),
        }
    }

    fn detect_resource_root(&self, module_path: &str, module: &mut Option<Module>) {
        let path = module_path;
        let resource_root = find_in_path(path, vec!["src", "main", "resources"]);
        match resource_root {
            Some(resource_root) => module.as_mut().unwrap().add_resource_root(resource_root),
            _ => (),
        }
    }

    fn detect_test_source_root(&self, module_path: &str, module: &mut Option<Module>) {
        let path = module_path;
        let test_source_root = find_in_path(path, vec!["src", "test", "java"]);
        match test_source_root {
            Some(test_source_root) => module
                .as_mut()
                .unwrap()
                .add_test_source_root(test_source_root),
            _ => (),
        }
    }

    fn detect_test_resource_root(&self, module_path: &str, module: &mut Option<Module>) {
        let path = module_path;
        let test_resource_root = find_in_path(path, vec!["src", "test", "resources"]);
        match test_resource_root {
            Some(test_resource_root) => module
                .as_mut()
                .unwrap()
                .add_test_resource_root(test_resource_root),
            _ => (),
        }
    }
}

impl ModuleAnalyzer for MavenModuleAnalyzer {
    fn analysis(&self, module_path: &str) -> Option<Module> {
        let mut module = create_module(module_path);
        if !module.is_none() {
            self.detect_sub_modules(&module_path, &mut module);
            self.detect_content_root(module_path, &mut module);
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
