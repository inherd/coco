use std::collections::BTreeMap;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

pub mod go;
pub mod js;
pub mod jvm;
pub mod rust;

type TagAction<'a> = fn(dir: &DirEntry) -> Option<&'a str>;

struct LangDetector<'a> {
    tag_action: TagAction<'a>,
}

pub struct LangDetectors<'a> {
    pub tags: BTreeMap<&'a str, bool>,
    detectors: Vec<LangDetector<'a>>,
}

impl<'a> Default for LangDetectors<'a> {
    fn default() -> Self {
        LangDetectors {
            tags: BTreeMap::default(),
            detectors: vec![
                LangDetector {
                    tag_action: jvm::get_tag,
                },
                LangDetector {
                    tag_action: js::get_tag,
                },
                LangDetector {
                    tag_action: go::get_tag,
                },
                LangDetector {
                    tag_action: rust::get_tag,
                },
            ],
        }
    }
}

impl<'a> LangDetectors<'a> {
    pub fn detect<P: AsRef<Path>>(&mut self, path: P) {
        traverse_project_directory(path, |dir_entry| {
            for detector in self.detectors.iter() {
                match (detector.tag_action)(dir_entry) {
                    Some(tag) => {
                        self.tags.insert(tag, true);
                    }
                    _ => continue,
                }
            }
        })
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
