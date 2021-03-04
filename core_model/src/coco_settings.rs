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

/// a collections of dir for projects
/// - `root()`, which is `.coco`
/// - `reporter()`, which is `.coco/reporter`
/// under `.coco/reporter`
/// - `architecture()`
/// - `git()`
/// - `cloc()`
/// - `framework()`
pub struct Settings {}

impl Settings {
    pub fn global_config(key: &str) -> &'static str {
        return COCO_CONFIG.get(key).unwrap();
    }

    pub fn root() -> &'static str {
        return Settings::global_config("dir");
    }

    pub fn reporter(child: Option<&str>) -> PathBuf {
        let root = Path::new(Settings::root());
        let reporter_path = root.join("reporter");
        if !reporter_path.exists() {
            let _ = fs::create_dir_all(&reporter_path);
        }

        match child {
            None => reporter_path,
            Some(str) => {
                let child_path = reporter_path.join(str);
                if !child_path.exists() {
                    let _ = fs::create_dir_all(&child_path);
                }

                child_path
            }
        }
    }

    pub fn git() -> PathBuf {
        Settings::reporter(Some("git"))
    }

    pub fn cloc() -> PathBuf {
        Settings::reporter(Some("cloc"))
    }

    pub fn architecture() -> PathBuf {
        Settings::reporter(Some("architecture"))
    }

    pub fn framework() -> PathBuf {
        Settings::reporter(Some("framework"))
    }

    pub fn pipeline() -> PathBuf {
        Settings::reporter(Some("pipeline"))
    }

    pub fn struct_dir() -> PathBuf {
        Settings::reporter(Some("struct"))
    }
}
