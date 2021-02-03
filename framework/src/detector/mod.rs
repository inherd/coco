pub use framework_detector::FrameworkDetector;
use std::collections::{BTreeMap, HashSet};

pub mod framework_detector;
pub mod lang;

type LightDetect = fn(HashSet<&str>) -> BTreeMap<&str, bool>;

struct LangDetector {
    light: LightDetect,
}

struct LangDetectors {
    detectors: Vec<Box<LangDetector>>,
}

impl LangDetectors {
    pub fn new() -> Self {
        LangDetectors { detectors: vec![] }
    }
}
