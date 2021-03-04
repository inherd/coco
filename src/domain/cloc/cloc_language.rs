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
    /// Total blank lines
    pub blanks: usize,
    /// Total number of lines within the file.
    pub code: usize,
    /// Number of comments within the file.
    pub comments: usize,
    /// File name
    pub file_name: String,
    /// really path
    pub path: String,
    /// File size in bytes
    pub bytes: u64,
}

impl Default for ClocDetail {
    fn default() -> Self {
        ClocDetail {
            blanks: 0,
            code: 0,
            comments: 0,
            file_name: "".to_string(),
            path: "".to_string(),
            bytes: 0,
        }
    }
}
