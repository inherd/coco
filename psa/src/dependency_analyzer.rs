use crate::files::list_file_names;
use crate::Dependency;
use std::path::Path;

pub trait DependencyAnalyzer {
    fn analysis(&self, module_path: &str) -> Vec<Dependency> {
        let build_file = self.get_build_file(module_path);
        match build_file {
            Some(build_file) => self.analysis_dependencies(module_path, build_file.as_str()),
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

    fn analysis_dependencies(&self, module_path: &str, build_file: &str) -> Vec<Dependency>;
}
