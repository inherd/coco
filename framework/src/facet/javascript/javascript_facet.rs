#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct JavaScriptFacet {
    pub is_frontend: bool,
    pub is_angular: bool,
    pub is_react: bool,
    pub is_vue: bool,

    pub is_node: bool,
    pub is_browser: bool,
    pub is_ionic: bool,
    pub is_cordova: bool,
    pub is_bower: bool,
    pub is_hybrid: bool,

    pub is_typescript: bool,
    pub is_javascript: bool,

    pub has_grunt: bool,
    pub has_gulp: bool,
}

impl Default for JavaScriptFacet {
    fn default() -> Self {
        JavaScriptFacet {
            is_frontend: false,
            is_angular: false,
            is_react: false,
            is_vue: false,
            is_node: false,
            is_browser: false,
            is_ionic: false,
            is_cordova: false,
            is_bower: false,
            is_hybrid: false,
            is_typescript: false,
            is_javascript: false,
            has_grunt: false,
            has_gulp: false,
        }
    }
}
