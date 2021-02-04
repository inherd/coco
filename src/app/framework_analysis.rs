use framework::framework_detector::FrameworkDetector;
use std::path::PathBuf;

pub fn analysis(path: PathBuf) -> String {
    let mut detector = FrameworkDetector::new();
    detector.run(path);

    return serde_json::to_string_pretty(&detector).unwrap();
}

#[cfg(test)]
mod test {
    use crate::app::framework_analysis;
    use std::path::PathBuf;

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
        println!("{}", result);
        assert_ne!("", result);
    }
}
