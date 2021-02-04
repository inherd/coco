use std::collections::{BTreeMap, HashSet};

pub mod go;
pub mod java;
pub mod js;
pub mod rust;

type LightDetect<'a> = fn(&HashSet<String>) -> BTreeMap<&'a str, bool>;

struct LangDetector<'a> {
    light: LightDetect<'a>,
}

pub struct LangDetectors<'a> {
    detectors: Vec<LangDetector<'a>>,
}

impl<'a> LangDetectors<'a> {
    pub fn new() -> Self {
        LangDetectors {
            detectors: vec![
                LangDetector {
                    light: java::light_detect,
                },
                LangDetector {
                    light: go::light_detect,
                },
                LangDetector {
                    light: rust::light_detect,
                },
                LangDetector {
                    light: js::light_detect,
                },
            ],
        }
    }

    pub fn light_detect(&self, names: &HashSet<String>) -> BTreeMap<&'a str, bool> {
        let mut tags = BTreeMap::new();
        for detector in self.detectors.iter() {
            let mut lang_tags = (detector.light)(names);
            tags.append(&mut lang_tags)
        }
        tags
    }
}
