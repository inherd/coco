use crate::framework_detector::Frameworks;
use std::collections::BTreeMap;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

pub mod go;
pub mod js;
pub mod jvm;
pub mod rust;

type TaggingAction<'a> = fn(dir: &DirEntry) -> Option<&'a str>;
type FrameworkAnalysisAction = fn(dir: &DirEntry, frameworks: &Frameworks);

struct LangDetector<'a> {
    tagging: TaggingAction<'a>,
    framework_analysis: FrameworkAnalysisAction,
}

pub struct LangDetectors<'a> {
    pub tags: BTreeMap<&'a str, bool>,
    pub frameworks: Frameworks,
    detectors: Vec<LangDetector<'a>>,
}

impl<'a> Default for LangDetectors<'a> {
    fn default() -> Self {
        LangDetectors {
            tags: BTreeMap::default(),
            detectors: vec![
                LangDetector {
                    tagging: jvm::get_tag,
                    framework_analysis: jvm::framework_analysis,
                },
                LangDetector {
                    tagging: js::get_tag,
                    framework_analysis: js::framework_analysis,
                },
                LangDetector {
                    tagging: go::get_tag,
                    framework_analysis: go::framework_analysis,
                },
                LangDetector {
                    tagging: rust::get_tag,
                    framework_analysis: rust::framework_analysis,
                },
            ],
            frameworks: Frameworks::default(),
        }
    }
}

impl<'a> LangDetectors<'a> {
    pub fn detect<P: AsRef<Path>>(&mut self, path: P) {
        traverse_project_directory(path, |dir_entry| {
            self.tagging(dir_entry);
            self.framework_analysis(dir_entry);
        })
    }

    fn tagging(&mut self, dir_entry: &DirEntry) {
        for detector in self.detectors.iter() {
            match (detector.tagging)(dir_entry) {
                Some(tag) => {
                    self.tags.insert(tag, true);
                }
                _ => continue,
            }
        }
    }

    fn framework_analysis(&mut self, dir_entry: &DirEntry) {
        for detector in self.detectors.iter() {
            (detector.framework_analysis)(dir_entry, &mut self.frameworks);
        }
    }
}

fn traverse_project_directory<'a, P: AsRef<Path>, F>(path: P, mut each_entry_callback: F)
where
    F: FnMut(&DirEntry),
{
    let walk_dir = WalkDir::new(path);
    for dir_entry in walk_dir.into_iter() {
        if dir_entry.is_err() {
            continue;
        }

        let entry = dir_entry.unwrap();
        if entry.path().file_name().is_none() {
            println!("none file_name {:?}", entry.path());
            continue;
        }

        (each_entry_callback)(&entry)
    }
}
