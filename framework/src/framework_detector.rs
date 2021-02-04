use std::collections::{BTreeMap, HashSet};

use walkdir::WalkDir;

use crate::facet::{Facet, JavaFacet, JvmFacet};
use crate::lang::LangDetectors;
use std::path::Path;

#[derive(Serialize, PartialEq, Debug, Clone)]
pub struct Framework {
    pub name: String,
    pub path: String,
    // for find the projects
    pub relative_path: String,
    // in some languages has different framework file
    // |   languages |   files    |
    // |-------------|------------|
    // | Java        | build.gradle, settings.gradle |
    pub files: Vec<String>,
    // in JVM projects, has different languages, such as Java, Groovy, Kotlin...
    pub languages: Vec<String>,
}

#[derive(Serialize)]
pub struct FrameworkDetector<'a> {
    pub tags: BTreeMap<&'a str, bool>,
    pub frameworks: Vec<Framework>,
    pub java_facets: Vec<Box<Facet>>,
}

impl<'a> FrameworkDetector<'a> {
    pub fn new() -> Self {
        FrameworkDetector {
            tags: BTreeMap::default(),
            frameworks: vec![],
            java_facets: vec![],
        }
    }

    pub fn run<P: AsRef<Path>>(&mut self, path: P) {
        let detectors = LangDetectors::new();
        self.light_detector(&detectors, &path);
        self.deep_detector(&detectors, &path);
        self.build_project_info();
    }

    fn deep_detector<P: AsRef<Path>>(&mut self, _detectors: &LangDetectors<'a>, _path: &P) {
        // todo: thinking in merge with cloc?
    }

    fn build_project_info(&mut self) {
        if self.is_contains("workspace.java.gradle") || self.is_contains("workspace.java.pom") {
            let facet = JavaFacet {
                jvm: JvmFacet {
                    is_gradle: self.is_contains("workspace.java.gradle"),
                    is_maven: self.is_contains("workspace.java.pom"),
                    has_java: false,
                    has_groovy: false,
                    has_kotlin: false,
                    has_scala: false,
                },
                include_test: false,
            };

            self.java_facets.push(Box::new(facet));
        }
    }

    fn is_contains(&self, key: &str) -> bool {
        self.tags.contains_key(key)
    }

    fn light_detector<P: AsRef<Path>>(&mut self, detectors: &LangDetectors<'a>, path: &P) {
        let sets = FrameworkDetector::build_level_one_name_set(path);
        let mut lang_tags = detectors.light_detect(&sets);
        self.tags.append(&mut lang_tags);
    }

    pub fn build_level_one_name_set<P: AsRef<Path>>(path: P) -> HashSet<String> {
        let mut name_sets: HashSet<String> = HashSet::new();
        let walk_dir = WalkDir::new(path);
        for dir_entry in walk_dir.max_depth(1).into_iter() {
            if dir_entry.is_err() {
                continue;
            }

            let entry = dir_entry.unwrap();
            if entry.path().file_name().is_none() {
                println!("none file_name {:?}", entry.path());
                continue;
            }

            let file_name = entry.path().file_name().unwrap();
            name_sets.insert(file_name.to_str().unwrap().to_string());
        }

        name_sets
    }
}

#[cfg(test)]
mod tests {
    use crate::framework_detector::FrameworkDetector;
    use std::path::PathBuf;

    fn build_test_detector<'a>(project_path: Vec<&str>) -> FrameworkDetector<'a> {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let mut test_project_dir = root_dir.clone();

        for path in project_path.into_iter() {
            test_project_dir.push(path);
        }

        let mut detector = FrameworkDetector::new();
        detector.run(test_project_dir.display().to_string());
        detector
    }

    #[test]
    fn should_detect_java_gradle_project() {
        let detector = build_test_detector(vec!["_fixtures", "projects", "java", "simple"]);

        assert!(detector.tags.get("workspace.java.gradle").unwrap());
        assert!(detector
            .tags
            .get("workspace.java.gradle.composite")
            .unwrap());
        assert_eq!(&false, detector.tags.get("workspace.npm").unwrap());
    }

    #[test]
    fn should_build_framework_info() {
        let detector = build_test_detector(vec!["_fixtures", "projects", "java", "hello"]);

        assert_eq!(1, detector.java_facets.len());
    }

    #[test]
    fn should_detect_go_project() {
        let detector = build_test_detector(vec!["_fixtures", "projects", "go", "hello"]);
        assert_eq!(&true, detector.tags.get("workspace.go").unwrap());

        let detector = build_test_detector(vec!["_fixtures", "projects", "go", "simple"]);
        assert_eq!(&true, detector.tags.get("workspace.go").unwrap());
    }

    #[test]
    fn should_detect_rust_cargo_project() {
        let detector = build_test_detector(vec!["_fixtures", "projects", "rust", "cargo"]);

        assert_eq!(&true, detector.tags.get("workspace.rust.cargo").unwrap());
    }

    #[test]
    fn should_detect_bower_project() {
        let detector = build_test_detector(vec!["_fixtures", "projects", "js", "bowerproject"]);

        assert_eq!(&true, detector.tags.get("workspace.bower").unwrap());
    }

    #[test]
    fn should_detect_npm_project() {
        let detector = build_test_detector(vec!["_fixtures", "projects", "js", "npmproject"]);

        assert_eq!(&true, detector.tags.get("workspace.npm").unwrap());
    }
}
