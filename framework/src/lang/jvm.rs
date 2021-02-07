use regex::Regex;
use std::collections::{BTreeMap, HashSet};

pub const WORKSPACE_FRAMEWORK_GRADLE: &'static str = "workspace.framework.gradle";
pub const WORKSPACE_FRAMEWORK_GRADLE_COMPOSITE: &'static str =
    "workspace.framework.gradle.composite";
pub const WORKSPACE_FRAMEWORK_POM: &'static str = "workspace.framework.pom";

pub const WORKSPACE_HAS_TEST: &'static str = "workspace.source.test";
pub const WORKSPACE_SOURCE_JAVA: &'static str = "workspace.source.java";
pub const WORKSPACE_SOURCE_GROOVY: &'static str = "workspace.source.groovy";
pub const WORKSPACE_SOURCE_KOTLIN: &'static str = "workspace.source.kotlin";
pub const WORKSPACE_SOURCE_SCALA: &'static str = "workspace.source.scala";

lazy_static! {
    static ref JAVA_TEST: Regex = Regex::new(r".*(Tests|Test).java").unwrap();
    static ref JAVA_SOURCE_TEST: Regex = Regex::new(r".*.java").unwrap();
    static ref GROOVY_SOURCE_TEST: Regex = Regex::new(r".*.groovy").unwrap();
    static ref KOTLIN_SOURCE_TEST: Regex = Regex::new(r".*.kt").unwrap();
    static ref SCALA_SOURCE_TEST: Regex = Regex::new(r".*.scala").unwrap();
    static ref DETECT_LIST: Vec<(&'static str, fn(&str) -> bool)> = vec![
        (WORKSPACE_HAS_TEST, is_test),
        (WORKSPACE_SOURCE_JAVA, is_java_source_file),
        (WORKSPACE_SOURCE_GROOVY, is_groovy_source_file),
        (WORKSPACE_SOURCE_KOTLIN, is_kotlin_source_file),
        (WORKSPACE_SOURCE_SCALA, is_scala_source_file)
    ];
}

pub fn is_test(path: &str) -> bool {
    return JAVA_TEST.is_match(path);
}

pub fn is_java_source_file(path: &str) -> bool {
    return JAVA_SOURCE_TEST.is_match(path);
}

pub fn is_groovy_source_file(path: &str) -> bool {
    return GROOVY_SOURCE_TEST.is_match(path);
}

pub fn is_kotlin_source_file(path: &str) -> bool {
    return KOTLIN_SOURCE_TEST.is_match(path);
}

pub fn is_scala_source_file(path: &str) -> bool {
    return SCALA_SOURCE_TEST.is_match(path);
}

pub fn detect<'a>(file_names: &HashSet<String>) -> BTreeMap<&'a str, bool> {
    let mut tags = BTreeMap::new();
    detect_build_tool(file_names, &mut tags);
    detect_source_file(file_names, &mut tags);
    tags
}

fn detect_build_tool(names: &HashSet<String>, tags: &mut BTreeMap<&str, bool>) {
    tags.insert(WORKSPACE_FRAMEWORK_GRADLE, names.contains("build.gradle"));
    tags.insert(
        WORKSPACE_FRAMEWORK_GRADLE_COMPOSITE,
        names.contains("build.gradle") && names.contains("settings.gradle"),
    );
    tags.insert(WORKSPACE_FRAMEWORK_POM, names.contains("pom.xml"));
}

fn detect_source_file(file_names: &HashSet<String>, tags: &mut BTreeMap<&str, bool>) {
    //todo: optimize performance to remove duplicate detection?
    for file_name in file_names.iter() {
        for (key, detect_action) in DETECT_LIST.iter() {
            if (detect_action)(file_name) {
                tags.insert(key, true);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lang::jvm::{
        is_groovy_source_file, is_java_source_file, is_kotlin_source_file, is_scala_source_file,
        is_test,
    };

    #[test]
    fn should_ident_test_java_file() {
        assert_eq!(false, is_test("Hello.java"));
        assert_eq!(true, is_test("HelloWorldTest.java"));
        assert_eq!(true, is_test("HelloTests.java"));
    }

    #[test]
    fn should_ident_java_source_file() {
        assert_eq!(true, is_java_source_file("Hello.java"));
        assert_eq!(true, is_java_source_file("HelloWorldTest.java"));
    }

    #[test]
    fn should_ident_groovy_source_file() {
        assert_eq!(true, is_groovy_source_file("Hello.groovy"));
        assert_eq!(true, is_groovy_source_file("HelloTest.groovy"));
    }

    #[test]
    fn should_ident_kotlin_source_file() {
        assert_eq!(true, is_kotlin_source_file("Hello.kt"));
        assert_eq!(true, is_kotlin_source_file("HelloTest.kt"));
    }

    #[test]
    fn should_ident_scala_source_file() {
        assert_eq!(true, is_scala_source_file("Hello.scala"));
        assert_eq!(true, is_scala_source_file("HelloTest.scala"));
    }
}
