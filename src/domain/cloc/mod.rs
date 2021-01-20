use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ClocLanguage {
    pub language: String,
    pub blanks: usize,
    pub code: usize,
    pub comments: usize,
    pub reports: Vec<ClocDetail>,
}

impl Default for ClocLanguage {
    fn default() -> Self {
        ClocLanguage {
            language: "".to_string(),
            blanks: 0,
            code: 0,
            comments: 0,
            reports: vec![],
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ClocDetail {
    pub blanks: usize,
    pub code: usize,
    pub comments: usize,
    pub file_name: String,
    pub path: String,
}

impl Default for ClocDetail {
    fn default() -> Self {
        ClocDetail {
            blanks: 0,
            code: 0,
            comments: 0,
            file_name: "".to_string(),
            path: "".to_string(),
        }
    }
}
