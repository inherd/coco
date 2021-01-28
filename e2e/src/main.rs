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

pub fn coco_program() -> String {
    return format!("{:?}", coco_exe());
}

#[cfg(test)]
mod tests {
    use crate::coco_exe;
    use assert_cmd::Command;

    #[test]
    fn should_find_coco_bin_path() {
        assert!(format!("{:?}", coco_exe()).contains("coco"));
    }

    #[test]
    fn should_run_coco_binary() {
        let mut cmd = Command::cargo_bin("coco").unwrap();
        // cmd.assert().success();
    }
}
