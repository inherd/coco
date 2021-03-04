use crate::psa_project::Project;
use crate::{Module, ModuleAnalyzer};

pub trait ProjectStructureAnalyzer {
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
