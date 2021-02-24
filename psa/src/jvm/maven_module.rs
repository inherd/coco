use std::path::Path;

use crate::jvm::psa_jvm::ModuleAnalyzer;
use crate::{Module, Project};

pub struct MavenModuleAnalyzer {}

impl ModuleAnalyzer for MavenModuleAnalyzer {
    fn analysis(&self, module_path: &str) -> Module {
        let module_name = get_module_name(module_path);

        Module::new(module_name.as_str(), module_path)
    }

    fn is_related(&self, project: &Project) -> bool {
        project.project_type == "maven"
    }
}

fn get_module_name(project_path: &str) -> String {
    Path::new(project_path)
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap()
}
