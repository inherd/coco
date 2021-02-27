use std::path::Path;

pub use module_analyzer::ModuleAnalyzer;
pub use pas_content_root::ContentRoot;
pub use project_structure_analyzer::ProjectAnalyzer;
pub use psa_dependency::Dependency;
pub use psa_dependency::DependencyScope;
pub use psa_facet::Facet;
pub use psa_module::Module;
pub use psa_project::Project;

use crate::files::list_file_names;

#[macro_use]
extern crate serde_derive;
extern crate serde;

pub mod files;
pub mod jvm;
pub mod module_analyzer;
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

pub trait DependencyAnalyzer {
    fn analysis(&self, module_path: &str) -> Vec<Dependency> {
        let build_file = self.get_build_file(module_path);
        match build_file {
            Some(build_file) => self.analysis_dependencies(build_file.as_str()),
            _ => vec![],
        }
    }

    fn get_build_file(&self, module_path: &str) -> Option<String> {
        let file_names = list_file_names(Path::new(module_path));
        file_names
            .iter()
            .find(|file| self.is_build_file(file.as_str()))
            .map(|file| file.to_string())
    }

    fn is_build_file(&self, file: &str) -> bool;

    fn analysis_dependencies(&self, build_file: &str) -> Vec<Dependency>;
}
