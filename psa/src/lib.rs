pub use pas_content_root::ContentRoot;
pub use project_structure_analyzer::ProjectAnalyzer;
pub use psa_facet::Facet;
pub use psa_library::Library;
pub use psa_library::LibraryScope;
pub use psa_module::Module;
pub use psa_project::Project;

#[macro_use]
extern crate serde_derive;
extern crate serde;

pub mod files;
pub mod jvm;
pub mod pas_content_root;
pub mod project_structure_analyzer;
pub mod psa_facet;
pub mod psa_library;
pub mod psa_module;
pub mod psa_project;

trait ProjectStructureAnalyzer {
    fn analysis(&self, project_path: &str) -> Project {
        let project_name = self.get_project_name(project_path);
        let project_type = self.get_project_type();

        let mut project = Project::new(project_name.as_str(), project_path, project_type.as_str());
        let project_module = self.analysis_project_module(&project);
        match project_module {
            Some(module) => project.set_project_module(module),
            _ => (),
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
    fn analysis(&self, project_path: &str, module_path: &str) -> Option<Module>;
    fn is_related(&self, project: &Project) -> bool;
}
