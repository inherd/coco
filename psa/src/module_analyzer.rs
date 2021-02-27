use crate::files::{list_sub_dirs, to_relative_path};
use crate::{ContentRoot, Dependency, DependencyAnalyzer, Module, Project};
use std::path::Path;

pub trait ModuleAnalyzer {
    fn analysis(&self, project_path: &str, module_path: &str) -> Option<Module> {
        let mut module = self.create_module(project_path, module_path);

        if let Some(module) = module.as_mut() {
            let sub_module = &mut self.detect_sub_modules(project_path, &module_path);
            module.add_sub_modules(sub_module);

            let content_root = self.detect_content_root(module_path);
            module.set_content_root(content_root);

            let dependencies = &mut self.analysis_dependencies(module_path);
            module.add_dependencies(dependencies);
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

    fn analysis_dependencies(&self, module_path: &str) -> Vec<Dependency> {
        let dependency_analyzer = self.get_dependency_analyzer();

        dependency_analyzer.analysis(module_path)
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
    fn get_dependency_analyzer(&self) -> Box<dyn DependencyAnalyzer>;
}
