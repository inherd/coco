pub use pas_content_root::ContentRoot;
pub use project_structure_analyzer::ProjectAnalyzer;
pub use psa_facet::Facet;
pub use psa_library::Library;
pub use psa_library::LibraryScope;
pub use psa_module::Module;
pub use psa_project::Project;

pub mod files;
pub mod jvm;
pub mod pas_content_root;
pub mod project_structure_analyzer;
pub mod psa_facet;
pub mod psa_library;
pub mod psa_module;
pub mod psa_project;

pub trait ProjectStructureAnalyzer {
    fn analysis(&self, project_path: &str) -> Project;
    fn is_related(&self, project_path: &str) -> bool;
}
