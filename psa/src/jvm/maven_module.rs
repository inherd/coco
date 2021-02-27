use std::path::Path;

use crate::files::{find_in_path, list_file_names};
use crate::jvm::maven_dependency::MavenDependencyAnalyzer;
use crate::Project;
use crate::{DependencyAnalyzer, ModuleAnalyzer};

pub struct MavenModuleAnalyzer {}

impl ModuleAnalyzer for MavenModuleAnalyzer {
    fn has_build_file(&self, module_path: &str) -> bool {
        let file_names = list_file_names(module_path);

        for file_name in file_names.iter() {
            return match file_name.as_str() {
                "pom.xml" => true,
                _ => continue,
            };
        }

        false
    }

    fn get_module_name(&self, project_path: &str) -> String {
        Path::new(project_path)
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap()
    }

    fn is_related(&self, project: &Project) -> bool {
        project.project_type == "maven"
    }

    fn get_source_root(&self, module_path: &str) -> Option<String> {
        find_in_path(module_path, vec!["src", "main", "java"])
    }

    fn get_resource_root(&self, module_path: &str) -> Option<String> {
        find_in_path(module_path, vec!["src", "main", "resources"])
    }

    fn get_test_source_root(&self, module_path: &str) -> Option<String> {
        find_in_path(module_path, vec!["src", "test", "java"])
    }

    fn get_test_resource_root(&self, module_path: &str) -> Option<String> {
        find_in_path(module_path, vec!["src", "test", "resources"])
    }

    fn get_dependency_analyzer(&self) -> Box<dyn DependencyAnalyzer> {
        Box::new(MavenDependencyAnalyzer {})
    }
}
