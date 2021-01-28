use std::env;
use std::path::PathBuf;

fn main() {
    println!("Hello, world!");
}

// Path to cargo executables
pub fn cargo_dir() -> PathBuf {
    env::var_os("CARGO_BIN_PATH")
        .map(PathBuf::from)
        .or_else(|| {
            env::current_exe().ok().map(|mut path| {
                path.pop();
                if path.ends_with("deps") {
                    path.pop();
                }
                path
            })
        })
        .unwrap_or_else(|| panic!("CARGO_BIN_PATH wasn't set. Cannot continue running test"))
}

pub fn coco_exe() -> PathBuf {
    cargo_dir().join(format!("coco{}", env::consts::EXE_SUFFIX))
}

#[cfg(test)]
mod tests {
    use crate::coco_exe;

    #[test]
    fn should_find_coco_bin_path() {
        assert!(format!("{:?}", coco_exe()).contains("coco"));
    }
}
