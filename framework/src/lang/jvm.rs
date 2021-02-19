use crate::framework_detector::{Framework, Frameworks};
use regex::Regex;
use std::cell::RefCell;
use walkdir::DirEntry;

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
    static ref SOURCE_DETECT_LIST: Vec<(&'static str, fn(&str) -> bool)> = vec![
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

pub fn get_tag<'a>(entry: &DirEntry) -> Option<&'a str> {
    let file_name = entry.file_name().to_str().unwrap();
    let build_tool_tag = get_build_tool_tag(file_name);
    if !build_tool_tag.is_none() {
        return build_tool_tag;
    }
    get_source_tag(file_name)
}

pub fn framework_analysis(entry: &DirEntry, frameworks: &Frameworks) {
    let file_name = entry.file_name().to_str().unwrap();
    let parent_path = entry.path().parent().unwrap().to_str().unwrap();

    if is_build_file(file_name) {
        frameworks.add_framework(Framework {
            name: ident_framework_name(file_name).to_string(),
            path: entry.path().parent().unwrap().to_str().unwrap().to_string(),
            files: RefCell::new(hashset! {file_name.to_string()}),
            languages: RefCell::new(hashset! {}),
        });
    }

    if is_build_settings_file(file_name) {
        let framework_name = get_settings_file_framework_name(file_name);

        frameworks.add_settings_file(framework_name, parent_path, file_name);
    }

    if is_source_file(file_name) {
        let language = ident_language(file_name);

        match language {
            Some(lang) => frameworks.add_language(parent_path, lang),
            _ => {}
        }
    }
}

fn get_settings_file_framework_name(file_name: &str) -> &str {
    match file_name {
        "settings.gradle" => "Gradle",
        _ => "",
    }
}

fn is_build_settings_file(file_name: &str) -> bool {
    match file_name {
        "settings.gradle" => true,
        _ => false,
    }
}

fn ident_language(file_name: &str) -> Option<&str> {
    if is_java_source_file(file_name) {
        return Some("Java");
    }

    if is_kotlin_source_file(file_name) {
        return Some("Kotlin");
    }

    if is_groovy_source_file(file_name) {
        return Some("Groovy");
    }

    if is_scala_source_file(file_name) {
        return Some("Scala");
    }

    None
}

fn is_source_file(file_name: &str) -> bool {
    for (_, detect_action) in SOURCE_DETECT_LIST.iter() {
        if (detect_action)(file_name) {
            return true;
        }
    }
    false
}

fn ident_framework_name(build_file: &str) -> &str {
    match build_file {
        "pom.xml" => "Maven",
        "build.gradle" => "Gradle",
        _ => "UnKnow",
    }
}

fn is_build_file(file_name: &str) -> bool {
    match file_name {
        "build.gradle" => true,
        "pom.xml" => true,
        _ => false,
    }
}

fn get_source_tag<'a>(file_name: &str) -> Option<&'a str> {
    for (key, detect_action) in SOURCE_DETECT_LIST.iter() {
        if (detect_action)(file_name) {
            return Some(key);
        }
    }
    None
}

fn get_build_tool_tag<'a>(file_name: &str) -> Option<&'a str> {
    match file_name {
        "build.gradle" => Some(WORKSPACE_FRAMEWORK_GRADLE),
        "settings.gradle" => Some(WORKSPACE_FRAMEWORK_GRADLE_COMPOSITE),
        "pom.xml" => Some(WORKSPACE_FRAMEWORK_POM),
        _ => None,
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
