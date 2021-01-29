use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct CodeRelation {
    // thinking in to file name
    pub id: String,
    pub path: String,
    pub source: String,
    pub target: String,
}
