#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct JvmFacet {
    pub is_gradle: bool,
    pub is_maven: bool,

    pub has_java: bool,
    pub has_groovy: bool,
    pub has_kotlin: bool,
    pub has_scala: bool,
}

impl Default for JvmFacet {
    fn default() -> Self {
        JvmFacet {
            is_gradle: false,
            is_maven: false,
            has_java: false,
            has_groovy: false,
            has_kotlin: false,
            has_scala: false,
        }
    }
}
