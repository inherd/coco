use crate::project_structure_analyzer::StructureAnalyzer;
use crate::psa_project::Project;
use crate::Module;
use std::path::Path;
use walkdir::WalkDir;

trait ModuleAnalyzer {
    fn analysis(module_path: &str) -> Option<Module>;
    fn is_related(&self, project: &Project) -> bool;
}

pub struct JvmProjectStructureAnalyzer {}

impl JvmProjectStructureAnalyzer {
    fn analysis_modules(&self) -> Vec<Module> {
        vec![
            Module::new("multi_mod_maven_project", "foo"),
            Module::new("module1", "foo"),
            Module::new("module2", "foo"),
        ]
    }
}

impl Default for JvmProjectStructureAnalyzer {
    fn default() -> Self {
        JvmProjectStructureAnalyzer {}
    }
}

impl StructureAnalyzer for JvmProjectStructureAnalyzer {
    fn analysis(&self, project_path: &str) -> Project {
        let project_name = get_project_name(project_path);
        let build_file = get_build_file(project_path).unwrap();
        let project_type = get_project_type(build_file);

        let mut project = Project::new(project_name.as_str(), project_path, project_type.as_str());
        let modules = &mut self.analysis_modules();
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

#[allow(dead_code)]
fn first_level_dirs<P: AsRef<Path>>(path: P) -> Vec<String> {
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
    use crate::jvm::psa_jvm::JvmProjectStructureAnalyzer;
    use crate::project_structure_analyzer::StructureAnalyzer;
    use std::path::PathBuf;

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

        assert_eq!(project.project_type, "maven");
        assert_eq!(project.modules.len(), 3);
    }
}
