use crate::infrastructure::cloc;
use std::path::PathBuf;

pub fn analysis(path: PathBuf) -> String {
    cloc::by_dir(path);
    return "{}".to_string();
}
