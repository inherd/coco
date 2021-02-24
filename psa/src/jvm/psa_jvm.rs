use std::path::Path;

use walkdir::WalkDir;

use crate::jvm::maven_module::MavenModuleAnalyzer;
use crate::psa_project::Project;
use crate::{Module, ProjectStructureAnalyzer};

pub trait ModuleAnalyzer {
    fn analysis(&self, module_path: &str) -> Module;
    fn is_related(&self, project: &Project) -> bool;
}

pub struct JvmProjectStructureAnalyzer {
    module_analyzers: Vec<Box<dyn ModuleAnalyzer>>,
}

impl JvmProjectStructureAnalyzer {
    fn analysis_modules(&self, project: &Project) -> Vec<Module> {
        let mut modules = Vec::new();
        let dirs = list_dirs(Path::new(&project.path));
        for each_dir in dirs.iter() {
            let module = self.analysis_module(project, each_dir);
            match module {
                Some(module) => modules.push(module),
                _ => continue,
            }
        }
        modules
    }

    fn analysis_module(&self, project: &Project, module_path: &String) -> Option<Module> {
        for module_analyzer in self.module_analyzers.iter() {
            return match module_analyzer.is_related(project) {
                true => Some(module_analyzer.analysis(module_path)),
                _ => continue,
            };
        }
        None
    }
}

impl Default for JvmProjectStructureAnalyzer {
    fn default() -> Self {
        JvmProjectStructureAnalyzer {
            module_analyzers: vec![Box::new(MavenModuleAnalyzer {})],
        }
    }
}

impl ProjectStructureAnalyzer for JvmProjectStructureAnalyzer {
    fn analysis(&self, project_path: &str) -> Project {
        let project_name = get_project_name(project_path);
        let build_file = get_build_file(project_path).unwrap();
        let project_type = get_project_type(build_file);

        let mut project = Project::new(project_name.as_str(), project_path, project_type.as_str());
        let modules = &mut self.analysis_modules(&project);
        project.add_modules(modules);
        project
    }

    fn is_related(&self, project_path: &str) -> bool {
        let files = list_file_names(project_path);
        for file_name in files.iter() {
            if is_build_file(file_name) {
                return true;
            }
        }
        false
    }
}

fn get_project_type(build_file: String) -> String {
    return match build_file.as_str() {
        "pom.xml" => "maven".to_string(),
        _ => "UnKnow".to_string(),
    };
}

fn get_build_file(path: &str) -> Option<String> {
    let files = list_file_names(Path::new(path));
    files.into_iter().find(|file| is_build_file(file))
}

fn get_project_name(project_path: &str) -> String {
    Path::new(project_path)
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap()
}

fn is_build_file(file_name: &str) -> bool {
    match file_name {
        "pom.xml" => true,
        "build.gradle" => true,
        _ => false,
    }
}

fn list_file_names<P: AsRef<Path>>(path: P) -> Vec<String> {
    let mut files = Vec::new();
    let walk_dir = WalkDir::new(path);
    for dir_entry in walk_dir.max_depth(1).into_iter() {
        if dir_entry.is_err() {
            panic!("{}", dir_entry.err().unwrap());
        }

        let entry = dir_entry.unwrap();
        if entry.metadata().unwrap().is_file() {
            files.push(entry.file_name().to_os_string().into_string().unwrap());
        }
    }
    files
}

fn list_dirs<P: AsRef<Path>>(path: P) -> Vec<String> {
    let mut dirs = Vec::new();
    let walk_dir = WalkDir::new(path);
    for dir_entry in walk_dir
        .max_depth(1)
        .sort_by(|a, b| a.file_name().cmp(b.file_name()))
        .into_iter()
    {
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
    use std::path::PathBuf;

    use crate::jvm::psa_jvm::JvmProjectStructureAnalyzer;
    use crate::ProjectStructureAnalyzer;

    #[test]
    fn should_analysis_maven_project_sub_modules() {
        let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf()
            .join("_fixtures")
            .join("projects")
            .join("java")
            .join("multi_mod_maven_project")
            .clone();

        let analyzer = JvmProjectStructureAnalyzer::default();

        let project = analyzer.analysis(project_dir.display().to_string().as_str());

        let modules = project.modules;
        let project_module = modules.get(0).unwrap();
        let module1 = modules.get(1).unwrap();
        let module2 = modules.get(2).unwrap();
        assert_eq!(modules.len(), 3);
        assert_eq!(project.project_type, "maven");
        assert_eq!(project_module.name, "multi_mod_maven_project");
        assert_eq!(module1.name, "module1");
        assert_eq!(module2.name, "module2");
    }
}
