use crate::files::{list_sub_dirs, to_relative_path};
pub use pas_content_root::ContentRoot;
pub use project_structure_analyzer::ProjectAnalyzer;
pub use psa_dependency::Dependency;
pub use psa_dependency::DependencyScope;
pub use psa_facet::Facet;
pub use psa_module::Module;
pub use psa_project::Project;
use std::path::Path;

#[macro_use]
extern crate serde_derive;
extern crate serde;

pub mod files;
pub mod jvm;
pub mod pas_content_root;
pub mod project_structure_analyzer;
pub mod psa_dependency;
pub mod psa_facet;
pub mod psa_module;
pub mod psa_project;

trait ProjectStructureAnalyzer {
    fn analysis(&self, project_path: &str) -> Project {
        let project_name = self.get_project_name(project_path);
        let project_type = self.get_project_type();

        let mut project = Project::new(project_name.as_str(), project_path, project_type.as_str());

        if let Some(project_module) = self.analysis_project_module(&project) {
            project.set_project_module(project_module)
        }

        project
    }

    fn analysis_project_module(&self, project: &Project) -> Option<Module> {
        for module_analyzer in self.get_module_analyzers().iter() {
            return match module_analyzer.is_related(project) {
                true => module_analyzer.analysis(&project.absolute_path, &project.absolute_path),
                _ => continue,
            };
        }
        None
    }

    fn get_project_name(&self, project_path: &str) -> String;
    fn get_project_type(&self) -> String;
    fn is_related(&self, project_path: &str) -> bool;
    fn get_module_analyzers(&self) -> Vec<Box<dyn ModuleAnalyzer>>;
}

pub trait ModuleAnalyzer {
    fn analysis(&self, project_path: &str, module_path: &str) -> Option<Module> {
        let mut module = self.create_module(project_path, module_path);

        if let Some(module) = module.as_mut() {
            let sub_module = &mut self.detect_sub_modules(project_path, &module_path);
            module.add_sub_modules(sub_module);

            let content_root = self.detect_content_root(module_path);
            module.set_content_root(content_root);
        }

        module
    }

    fn create_module(&self, project_path: &str, module_path: &str) -> Option<Module> {
        let module_name = self.get_module_name(module_path);
        let relative_path = to_relative_path(project_path, module_path);

        match self.has_build_file(module_path) {
            true => Some(Module::new(module_name.as_str(), relative_path.as_str())),
            _ => None,
        }
    }

    fn detect_sub_modules(&self, project_path: &str, module_path: &str) -> Vec<Module> {
        let mut sub_modules = Vec::new();

        let sub_dirs = list_sub_dirs(Path::new(module_path));

        for each_sub_dir in sub_dirs.iter() {
            let sub_module = self.analysis(project_path, each_sub_dir);
            match sub_module {
                Some(sub_module) => sub_modules.push(sub_module),
                _ => continue,
            }
        }

        sub_modules
    }

    fn detect_content_root(&self, module_path: &str) -> ContentRoot {
        let mut content_root = ContentRoot::default();

        if let Some(source_root) = self.detect_source_root(module_path) {
            content_root.add_source_root(source_root.as_str())
        }

        if let Some(resource_root) = self.detect_resource_root(module_path) {
            content_root.add_resource_root(resource_root.as_str())
        }

        if let Some(test_source_root) = self.detect_test_source_root(module_path) {
            content_root.add_test_source_root(test_source_root.as_str())
        }

        if let Some(test_resource_root) = self.detect_test_resource_root(module_path) {
            content_root.add_test_resource_root(test_resource_root.as_str())
        }

        content_root
    }

    fn detect_source_root(&self, module_path: &str) -> Option<String> {
        match self.get_source_root(module_path) {
            Some(source_root) => Some(to_relative_path(module_path, source_root.as_str())),
            _ => None,
        }
    }

    fn detect_resource_root(&self, module_path: &str) -> Option<String> {
        match self.get_resource_root(module_path) {
            Some(resource_root) => Some(to_relative_path(module_path, resource_root.as_str())),
            _ => None,
        }
    }

    fn detect_test_source_root(&self, module_path: &str) -> Option<String> {
        match self.get_test_source_root(module_path) {
            Some(test_source_root) => {
                Some(to_relative_path(module_path, test_source_root.as_str()))
            }
            _ => None,
        }
    }

    fn detect_test_resource_root(&self, module_path: &str) -> Option<String> {
        match self.get_test_resource_root(module_path) {
            Some(test_resource_root) => {
                Some(to_relative_path(module_path, test_resource_root.as_str()))
            }
            _ => None,
        }
    }

    fn has_build_file(&self, module_path: &str) -> bool;
    fn get_module_name(&self, project_path: &str) -> String;
    fn is_related(&self, project: &Project) -> bool;
    fn get_source_root(&self, module_path: &str) -> Option<String>;
    fn get_resource_root(&self, module_path: &str) -> Option<String>;
    fn get_test_source_root(&self, module_path: &str) -> Option<String>;
    fn get_test_resource_root(&self, module_path: &str) -> Option<String>;
}
