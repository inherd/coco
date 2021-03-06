use crate::jvm::psa_jvm::JvmProjectStructureAnalyzer;
pub use dependency_analyzer::DependencyAnalyzer;
pub use module_analyzer::ModuleAnalyzer;
pub use pas_content_root::ContentRoot;
pub use project_structure_analyzer::ProjectStructureAnalyzer;
pub use psa_dependency::Dependency;
pub use psa_dependency::DependencyScope;
pub use psa_facet::Facet;
pub use psa_module::Module;
pub use psa_project::Project;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate sxd_document;
extern crate sxd_xpath;

pub mod dependency_analyzer;
pub mod files;
pub mod jvm;
pub mod module_analyzer;
pub mod pas_content_root;
pub mod project_structure_analyzer;
pub mod psa_dependency;
pub mod psa_facet;
pub mod psa_module;
pub mod psa_project;

pub struct ProjectAnalyzer {
    analyzers: Vec<Box<dyn ProjectStructureAnalyzer>>,
}

impl ProjectAnalyzer {
    pub fn run(&self, path: &str) -> Option<Project> {
        for analyzer in self.analyzers.iter() {
            return match analyzer.is_related(path) {
                true => Some(analyzer.analysis(path)),
                _ => continue,
            };
        }
        None
    }
}

impl Default for ProjectAnalyzer {
    fn default() -> Self {
        ProjectAnalyzer {
            analyzers: vec![Box::new(JvmProjectStructureAnalyzer::default())],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ProjectAnalyzer;
    use std::path::PathBuf;

    #[test]
    fn should_analysis_project() {
        let project_dir_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf()
            .join("_fixtures")
            .join("projects")
            .join("java")
            .join("simple")
            .clone();
        let analyzer = ProjectAnalyzer::default();
        let project_dir = project_dir_path.display().to_string();
        let project = analyzer.run(project_dir.as_str()).unwrap();

        assert_eq!(project.name, "simple");
        assert_eq!(project.absolute_path, project_dir.as_str());
    }

    #[test]
    fn should_return_none_when_build_file_not_exists() {
        let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf()
            .join("_fixtures")
            .join("projects")
            .join("java")
            .clone();

        let analyzer = ProjectAnalyzer::default();

        let project = analyzer.run(project_dir.display().to_string().as_str());

        assert_eq!(project.is_none(), true);
    }

    #[test]
    fn should_serialize() {
        let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf()
            .join("_fixtures")
            .join("projects")
            .join("java")
            .join("multi_mod_maven_project")
            .clone();

        let analyzer = ProjectAnalyzer::default();

        let project = analyzer
            .run(project_dir.display().to_string().as_str())
            .unwrap();

        let project_json = serde_json::to_string_pretty(&project).unwrap();

        println!("{}", project_json);
        assert_ne!(project_json, "");
    }
}
