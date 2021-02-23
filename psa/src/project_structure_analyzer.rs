use crate::jvm::psa_jvm::JvmProjectStructureAnalyzer;
use crate::psa_project::Project;

pub trait StructureAnalyzer {
    fn analysis(&self, project_path: &str) -> Project;
    fn is_related(&self) -> bool;
}

pub struct ProjectAnalyzer {
    analyzers: Vec<Box<dyn StructureAnalyzer>>,
}

impl ProjectAnalyzer {
    pub fn run(&self, path: &str) -> Vec<Project> {
        let mut projects = Vec::new();
        for analyzer in self.analyzers.iter() {
            match analyzer.is_related() {
                true => projects.push(analyzer.analysis(path)),
                _ => continue,
            }
        }
        projects
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

    #[test]
    fn should_run_analyzer() {
        let analyzer = ProjectAnalyzer::default();

        let projects = analyzer.run("");

        assert_eq!(projects.len(), 1);
        assert_eq!(projects.get(0).unwrap().name, "test".to_string())
    }
}
