use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct FileInclude {
    pub includes: Vec<CodeInclude>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct CodeInclude {
    pub kind: String,
    pub name: String,
}
