use std::collections::HashMap;

lazy_static! {
    static ref COCO_CONFIG: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("dir", ".coco");
        m
    };
}

pub struct Settings {}

impl Settings {
    pub fn global_config(key: &str) -> &'static str {
        return COCO_CONFIG.get(key).unwrap();
    }

    pub fn dir() -> &'static str {
        return Settings::global_config("dir");
    }
}
