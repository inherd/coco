use crate::facet::java::content_root::ContentRoot;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct JavaModuleData {
    pub module_name: String,
    pub content_roots: Vec<ContentRoot>,
}

impl Default for JavaModuleData {
    fn default() -> Self {
        JavaModuleData {
            module_name: "".to_string(),
            content_roots: vec![],
        }
    }
}
