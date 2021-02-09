use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CocoTag {
    pub name: String,
    pub display_name: String,
    pub commit_id: String,
    pub date: i64,
    pub share_index: i64,
}

impl Default for CocoTag {
    fn default() -> Self {
        CocoTag {
            name: "".to_string(),
            display_name: "".to_string(),
            commit_id: "".to_string(),
            date: 0,
            share_index: 0,
        }
    }
}
