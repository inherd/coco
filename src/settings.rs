use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

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

    pub fn root_dir() -> &'static str {
        return Settings::global_config("dir");
    }

    pub fn reporter_dir() -> PathBuf {
        let root = Path::new(Settings::root_dir());
        let reporter_buf = root.join("reporter");
        if !reporter_buf.exists() {
            let _ = fs::create_dir_all(reporter_buf.clone());
        }

        reporter_buf
    }
}
