use assert_cmd::Command;
use std::path::PathBuf;
use std::{env, fs};

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

pub fn coco_cmd(path: PathBuf) -> Command {
    let mut cmd = Command::cargo_bin("coco").unwrap();

    cmd.arg("-c")
        .arg(format!("{}", path.into_os_string().to_str().unwrap()));
    cmd
}

pub fn output_file(typ: &str) -> PathBuf {
    let mut output = PathBuf::from(".coco");
    output.push("reporter");
    output.push(typ);
    output.push("coco.fixtures.json");
    output
}

pub fn read_coco_reporter(typ: &str) -> String {
    fs::read_to_string(output_file(typ)).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{coco_cmd, coco_exe, read_coco_reporter};
    use assert_cmd::Command;
    use std::path::PathBuf;

    #[test]
    fn should_find_coco_bin_path() {
        assert!(format!("{:?}", coco_exe()).contains("coco"));
    }

    #[test]
    fn should_exe_coco_failure_when_in_e2e_path() {
        let mut cmd = Command::cargo_bin("coco").unwrap();
        cmd.assert().failure();
    }

    #[test]
    fn should_build_fixtures_code() {
        let mut path = PathBuf::from("_fixtures");
        path.push("coco-fixtures.yml");
        let mut cmd = coco_cmd(path);
        cmd.assert().success();

        assert!(read_coco_reporter("cloc").len() > 0);
        assert!(read_coco_reporter("architecture").len() > 0);
        assert!(read_coco_reporter("framework").len() > 0);
        assert!(read_coco_reporter("git").len() > 0);
    }
}
