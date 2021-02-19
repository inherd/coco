use crate::facet::{Facet, FacetsBuilder};
use crate::lang::LangDetectors;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Serialize, PartialEq, Debug, Clone)]
pub struct Framework {
    pub name: String,
    pub path: String,
    // in some languages has different framework file
    // |   languages |   files    |
    // |-------------|------------|
    // | Java        | build.gradle, settings.gradle |
    pub files: RefCell<Vec<String>>,
    // in JVM projects, has different languages, such as Java, Groovy, Kotlin...
    pub languages: RefCell<Vec<String>>,
}

#[derive(Serialize)]
struct SourceFile {
    file_path: String,
    language: String,
}

#[derive(Serialize)]
pub struct Frameworks {
    frameworks: RefCell<Vec<Framework>>,

    #[serde(skip_serializing)]
    temp_source_files: RefCell<Vec<SourceFile>>,
}

impl Frameworks {
    pub fn add_framework(&self, framework: Framework) {
        if !self.frameworks.borrow().contains(&framework) {
            self.associate_with_source_files(&framework);
            self.frameworks.borrow_mut().push(framework);
        }
    }

    fn associate_with_source_files(&self, framework: &Framework) {
        for temp_source_file in self.temp_source_files.borrow().iter() {
            if temp_source_file.file_path.starts_with(&framework.path) {
                framework
                    .languages
                    .borrow_mut()
                    .push(temp_source_file.language.clone());
            }
        }
    }

    pub fn add_language(&self, file_path: &str, language: &str) {
        self.add_language_to_frameworks(file_path, &language);
        self.cache_source_file(file_path, language);
    }

    fn add_language_to_frameworks(&self, file_path: &str, language: &&str) {
        for framework in self.frameworks.borrow_mut().iter() {
            if file_path.starts_with(&framework.path)
                && !framework.languages.borrow().contains(&language.to_string())
            {
                framework.languages.borrow_mut().push(language.to_string());
            }
        }
    }

    fn cache_source_file(&self, file_path: &str, language: &str) {
        self.temp_source_files.borrow_mut().push(SourceFile {
            file_path: file_path.to_string(),
            language: language.to_string(),
        });
    }

    pub fn append(&self, frameworks: &Frameworks) {
        self.frameworks
            .borrow_mut()
            .append(&mut frameworks.frameworks.borrow_mut())
    }

    pub fn add_settings_file(&self, framework_name: &str, file_path: &str, file_name: &str) {
        for framework in self.frameworks.borrow_mut().iter() {
            if file_path.starts_with(&framework.path) && framework.name.eq(framework_name) {
                framework.files.borrow_mut().push(file_name.to_string());
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<Framework> {
        let frameworks = self.frameworks.borrow();
        match frameworks.get(index) {
            Some(framework) => Some(framework.clone()),
            _ => None,
        }
    }
}

impl Default for Frameworks {
    fn default() -> Self {
        Frameworks {
            frameworks: RefCell::new(vec![]),
            temp_source_files: RefCell::new(vec![]),
        }
    }
}

#[derive(Serialize)]
pub struct FrameworkDetector<'a> {
    pub tags: BTreeMap<&'a str, bool>,
    pub frameworks: Frameworks,
    pub facets: Vec<Box<Facet>>,
}

impl<'a> Default for FrameworkDetector<'a> {
    fn default() -> Self {
        FrameworkDetector {
            tags: BTreeMap::default(),
            frameworks: Frameworks::default(),
            facets: vec![],
        }
    }
}

impl<'a> FrameworkDetector<'a> {
    pub fn run<P: AsRef<Path>>(&mut self, path: P) {
        let mut lang_detectors = FrameworkDetector::detect(&path);

        self.add_tags(&mut lang_detectors);
        self.add_frameworks(&mut lang_detectors);
        self.add_facets();
    }

    fn detect<P: AsRef<Path>>(path: P) -> LangDetectors<'a> {
        let mut lang_detectors = LangDetectors::default();
        lang_detectors.detect(&path);
        lang_detectors
    }

    fn add_tags(&mut self, detectors: &mut LangDetectors<'a>) {
        self.tags.append(&mut detectors.tags);
    }

    fn add_facets(&mut self) {
        let builder = FacetsBuilder::default();
        let mut facets = builder.build(&self.tags);
        self.facets.append(&mut facets);
    }

    fn add_frameworks(&mut self, detectors: &mut LangDetectors<'a>) {
        self.frameworks.append(&detectors.frameworks);
    }
}

#[cfg(test)]
mod tests {
    use crate::framework_detector::FrameworkDetector;
    use crate::lang::jvm;
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

        let mut detector = FrameworkDetector::default();
        detector.run(test_project_dir.display().to_string());
        detector
    }

    #[test]
    fn should_detect_java_gradle_project() {
        let detector = build_test_detector(vec!["_fixtures", "projects", "java", "simple"]);

        assert!(detector.tags.get(jvm::WORKSPACE_FRAMEWORK_GRADLE).unwrap());
        assert!(detector
            .tags
            .get(jvm::WORKSPACE_FRAMEWORK_GRADLE_COMPOSITE)
            .unwrap());
    }

    #[test]
    fn should_build_framework_info() {
        let detector = build_test_detector(vec!["_fixtures", "projects", "java", "hello"]);

        assert_eq!(1, detector.facets.len());
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

        assert_eq!(&true, detector.tags.get("workspace.cargo").unwrap());
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

    #[test]
    fn should_detect_jvm_project() {
        let detector = build_test_detector(vec!["_fixtures", "projects", "jvm"]);

        let facets_json = serde_json::to_string_pretty(&detector.facets).unwrap();
        let expect_json = r#"[
  {
    "jvm": {
      "is_gradle": true,
      "is_maven": true,
      "has_java": true,
      "has_groovy": true,
      "has_kotlin": true,
      "has_scala": true
    },
    "include_test": true
  }
]"#;
        assert_eq!(expect_json, facets_json)
    }

    #[test]
    fn should_detect_jvm_frameworks() {
        let detector = build_test_detector(vec!["_fixtures", "projects", "jvm"]);
        let frameworks = detector.frameworks;

        let framework = frameworks.get(0).unwrap();
        let name = framework.name.to_string();
        let files = framework.files.borrow();
        let languages = framework.languages.borrow();
        assert_eq!(name, "Gradle");
        assert_eq!(files.get(0).unwrap().as_str(), "build.gradle");
        assert_eq!(files.get(1).unwrap().as_str(), "settings.gradle");
        assert_eq!(languages.get(0).unwrap().as_str(), "Scala");
        assert_eq!(languages.get(1).unwrap().as_str(), "Groovy");
        assert_eq!(languages.get(2).unwrap().as_str(), "Kotlin");
        assert_eq!(languages.get(3).unwrap().as_str(), "Java");

        let framework = frameworks.get(1).unwrap();
        let name = framework.name.to_string();
        let files = framework.files.borrow();
        let languages = framework.languages.borrow();
        assert_eq!(name, "Maven");
        assert_eq!(files.get(0).unwrap().as_str(), "pom.xml");
        assert_eq!(languages.get(0).unwrap().as_str(), "Java");
        assert_eq!(languages.get(1).unwrap().as_str(), "Kotlin");
    }
}
