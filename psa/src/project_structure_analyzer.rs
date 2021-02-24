use crate::jvm::psa_jvm::JvmProjectStructureAnalyzer;
use crate::psa_project::Project;

pub trait StructureAnalyzer {
    fn analysis(&self, project_path: &str) -> Project;
    fn is_related(&self, project_path: &str) -> bool;
}

pub struct ProjectAnalyzer {
    analyzers: Vec<Box<dyn StructureAnalyzer>>,
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
        assert_eq!(project.path, project_dir.as_str());
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
}
