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

    use crate::app::framework_analysis;

    #[test]
    fn should_return_json() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();

        let test_project_dir = root_dir
            .clone()
            .join("_fixtures")
            .join("projects")
            .join("java")
            .join("simple");

        let result = framework_analysis::analysis(test_project_dir);
        assert_ne!("", result);
    }
}
