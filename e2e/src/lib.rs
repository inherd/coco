use std::env;
use std::path::PathBuf;

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
    use std::fs;
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
        let mut cmd = Command::cargo_bin("coco").unwrap();

        cmd.arg("-c")
            .arg(format!("{}", path.into_os_string().to_str().unwrap()));

        cmd.assert().success();

        let mut output = PathBuf::from(".coco");
        output.push("reporter");
        output.push("architecture");
        output.push("coco.fixtures.json");

        let result = fs::read_to_string(output).unwrap();
        assert!(result.len() > 0);
    }
}
