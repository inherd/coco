use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConventionalMessage {
    pub type_: String,
    pub scope: String,
    pub breaking: bool,
    pub subject: String,
}

impl Default for ConventionalMessage {
    fn default() -> Self {
        ConventionalMessage {
            type_: "".to_string(),
            scope: "".to_string(),
            breaking: false,
            subject: "".to_string(),
        }
    }
}
