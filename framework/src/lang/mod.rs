use std::collections::{BTreeMap, HashSet};

pub mod go;
pub mod js;
pub mod jvm;
pub mod rust;

type DetectAction<'a> = fn(&HashSet<String>) -> BTreeMap<&'a str, bool>;

pub struct LangDetectors<'a> {
    detectors: Vec<DetectAction<'a>>,
}

impl<'a> Default for LangDetectors<'a> {
    fn default() -> Self {
        LangDetectors {
            detectors: vec![
                jvm::light_detect,
                go::light_detect,
                rust::light_detect,
                js::light_detect,
            ],
        }
    }
}

impl<'a> LangDetectors<'a> {
    pub fn detect(&self, names: &HashSet<String>) -> BTreeMap<&'a str, bool> {
        let mut tags = BTreeMap::default();
        for detector in self.detectors.iter() {
            let mut lang_tags = (detector)(names);
            tags.append(&mut lang_tags)
        }
        tags
    }
}
