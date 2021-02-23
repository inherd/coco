use std::path::PathBuf;

use framework::framework_detector::FrameworkDetector;

pub fn analysis(path: PathBuf) -> String {
    let mut detector = FrameworkDetector::default();
    detector.run(path);

    return serde_json::to_string_pretty(&detector).unwrap();
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn should_return_json() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).to_path_buf();

        let test_project_dir = root_dir
            .clone()
            .join("_fixtures")
            .join("projects")
            .join("java")
            .join("simple");

        let result = analysis(test_project_dir);
        assert_ne!("", result);
    }
}
