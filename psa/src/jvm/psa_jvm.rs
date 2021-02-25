use std::path::Path;

use crate::jvm::maven_module::MavenModuleAnalyzer;
use crate::psa_project::Project;
use crate::{files, Module, ProjectStructureAnalyzer};

pub trait ModuleAnalyzer {
    fn analysis(&self, module_path: &str) -> Option<Module>;
    fn is_related(&self, project: &Project) -> bool;
}

pub struct JvmProjectStructureAnalyzer {
    module_analyzers: Vec<Box<dyn ModuleAnalyzer>>,
}

impl JvmProjectStructureAnalyzer {
    fn analysis_modules(&self, project: &Project) -> Vec<Module> {
        let mut modules = Vec::new();
        let module = self.analysis_module(project);
        match module {
            Some(module) => modules.push(module),
            _ => (),
        }
        modules
    }

    fn analysis_module(&self, project: &Project) -> Option<Module> {
        for module_analyzer in self.module_analyzers.iter() {
            return match module_analyzer.is_related(project) {
                true => module_analyzer.analysis(&project.path),
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
        let files = files::list_file_names(project_path);
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
    let files = files::list_file_names(Path::new(path));
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
        let module1 = project_module.sub_modules.get(0).unwrap();
        let module2 = project_module.sub_modules.get(1).unwrap();
        assert_eq!(modules.len(), 1);
        assert_eq!(project_module.sub_modules.len(), 2);
        assert_eq!(project.project_type, "maven");
        assert_eq!(project_module.name, "multi_mod_maven_project");
        assert_eq!(module1.name, "module1");
        assert_eq!(module2.name, "module2");
    }
}
