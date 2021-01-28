use assert_cmd::Command;
use std::fs;
use std::path::PathBuf;

pub struct CliSupport {}

impl CliSupport {
    pub fn command(path: PathBuf) -> Command {
        let mut cmd = Command::cargo_bin("coco").unwrap();

        cmd.arg("-c")
            .arg(format!("{}", path.into_os_string().to_str().unwrap()));
        cmd
    }

    pub fn output(typ: &str) -> PathBuf {
        let mut output = PathBuf::from(".coco");
        output.push("reporter");
        output.push(typ);
        output.push("coco.fixtures.json");
        output
    }

    pub fn read_reporter(typ: &str) -> String {
        fs::read_to_string(CliSupport::output(typ)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::CliSupport;
    use assert_cmd::Command;
    use std::path::PathBuf;

    #[test]
    fn should_exe_coco_failure_when_in_e2e_path() {
        let mut cmd = Command::cargo_bin("coco").unwrap();
        cmd.assert().failure();
    }

    #[test]
    fn should_build_fixtures_code() {
        let mut path = PathBuf::from("_fixtures");
        path.push("coco-fixtures.yml");
        let mut cmd = CliSupport::command(path);
        cmd.assert().success();

        assert!(CliSupport::read_reporter("cloc").len() > 0);
        assert!(CliSupport::read_reporter("architecture").len() > 0);
        assert!(CliSupport::read_reporter("framework").len() > 0);
        assert!(CliSupport::read_reporter("git").len() > 0);
    }
}
