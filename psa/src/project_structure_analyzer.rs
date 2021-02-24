use crate::jvm::psa_jvm::JvmProjectStructureAnalyzer;
use crate::psa_project::Project;
use std::path::Path;
use walkdir::WalkDir;

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

fn _first_level_dirs<P: AsRef<Path>>(path: P) -> Vec<String> {
    let mut dirs = Vec::new();
    let walk_dir = WalkDir::new(path);
    for dir_entry in walk_dir.max_depth(1).into_iter() {
        if dir_entry.is_err() {
            panic!("{}", dir_entry.err().unwrap());
        }

        let entry = dir_entry.unwrap();
        if entry.metadata().unwrap().is_dir() {
            dirs.push(entry.path().display().to_string())
        }
    }
    dirs
}

#[cfg(test)]
mod tests {
    use crate::ProjectAnalyzer;
    use std::path::PathBuf;

    #[test]
    fn should_run_analyzer() {
        let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf()
            .join("_fixtures")
            .join("projects")
            .join("java")
            .join("simple")
            .clone();
        let analyzer = ProjectAnalyzer::default();

        let project = analyzer
            .run(project_dir.display().to_string().as_str())
            .unwrap();

        assert_eq!(project.name, "simple");
        assert_eq!(project.path.contains("/projects/java/simple"), true);
    }
}
