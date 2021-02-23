use crate::project_structure_analyzer::StructureAnalyzer;
use crate::psa_project::Project;

pub struct JvmProjectStructureAnalyzer {}

impl Default for JvmProjectStructureAnalyzer {
    fn default() -> Self {
        JvmProjectStructureAnalyzer {}
    }
}

impl StructureAnalyzer for JvmProjectStructureAnalyzer {
    fn analysis(&self, _project_path: &str) -> Project {
        Project::new("test", "test/path")
    }

    fn is_related(&self) -> bool {
        true
    }
}
