use framework::detector::FrameworkDetector;
use std::path::PathBuf;

pub fn analysis(path: PathBuf) -> String {
    let mut detector = FrameworkDetector::new();
    detector.run(path);

    return serde_json::to_string_pretty(&detector).unwrap();
}

#[cfg(test)]
mod test {
    #[test]
    fn should_return_json() {
        // todo:
    }
}
