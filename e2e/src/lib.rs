use assert_cmd::Command;
use std::fs;
use std::path::PathBuf;

pub struct CliSupport {}

impl CliSupport {
    pub fn coco_run(path: PathBuf) -> Command {
        let mut cmd = CliSupport::coco();

        cmd.arg("-c")
            .arg(format!("{}", path.into_os_string().to_str().unwrap()));
        cmd
    }

    fn coco() -> Command {
        let cmd = Command::cargo_bin("coco").unwrap();
        cmd
    }

    pub fn visual() -> Command {
        let cmd = Command::cargo_bin("visual").unwrap();
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
    use std::process;
    use walkdir::WalkDir;

    #[test]
    fn should_exe_coco_failure_when_in_e2e_path() {
        let mut cmd = Command::cargo_bin("coco").unwrap();
        cmd.assert().failure();
    }

    #[test]
    fn should_build_fixtures_code() {
        let mut path = PathBuf::from("_fixtures");
        path.push("coco-fixtures.yml");
        let mut cmd = CliSupport::coco_run(path);
        cmd.assert().success();

        assert!(CliSupport::read_reporter("cloc").len() > 0);
        assert!(CliSupport::read_reporter("architecture").len() > 0);
        assert!(CliSupport::read_reporter("framework").len() > 0);
        assert!(CliSupport::read_reporter("git").len() > 0);
    }

    #[test]
    fn should_download_plugins() {
        let mut cmd = CliSupport::coco();
        cmd.arg("plugins");

        cmd.assert().success();

        let mut vec = vec![];
        for entry in WalkDir::new("coco_plugins") {
            let entry = entry.unwrap();
            if entry.path().is_file() {
                vec.push(entry);
            }
        }

        assert_eq!(4, vec.len());
    }

    #[test]
    fn should_pass_no_plugins() {
        let mut path = PathBuf::from("_fixtures");
        path.push("no-plugin.yml");
        let mut cmd = CliSupport::coco_run(path);
        cmd.assert().success();
    }

    #[test]
    fn should_run_export() {
        let mut cmd = CliSupport::visual();

        cmd.arg("export");
        cmd.arg("--name").arg("default");

        cmd.assert().success();
    }

    #[ignore]
    #[test]
    fn should_run_http_server() {
        let mut cmd = CliSupport::visual();
        cmd.arg("--name").arg("default");

        cmd.arg("server");
        cmd.arg("-p").arg("9000");

        cmd.assert().success();

        process::exit(1);
    }
}
