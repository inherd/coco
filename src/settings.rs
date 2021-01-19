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

    pub fn reporter_dir(child: Option<&str>) -> PathBuf {
        let root = Path::new(Settings::root_dir());
        let reporter_path = root.join("reporter");
        if !reporter_path.exists() {
            let _ = fs::create_dir_all(reporter_path.clone());
        }

        match child {
            None => reporter_path,
            Some(str) => {
                let child_path = reporter_path.join(str);
                if !child_path.exists() {
                    let _ = fs::create_dir_all(child_path.clone());
                }

                child_path
            }
        }
    }
}
