use crate::facet::java::jvm_facet::JvmFacet;
use crate::facet::Facet;
use regex::Regex;
use std::collections::BTreeMap;

lazy_static! {
    static ref JAVA_TEST: Regex = Regex::new(r".*(Tests|Test).java").unwrap();
    static ref MAVEN_TEST: Regex = Regex::new(r".*pom.xml").unwrap();
}

#[derive(Serialize)]
pub struct JavaFacet {
    pub jvm: JvmFacet,
    pub include_test: bool,
}

impl JavaFacet {
    pub fn new() -> JavaFacet {
        JavaFacet {
            jvm: Default::default(),
            include_test: false,
        }
    }
    pub fn is_test(path: &str) -> bool {
        return JAVA_TEST.is_match(path);
    }
    pub fn is_maven(path: &str) -> bool {
        return MAVEN_TEST.is_match(path);
    }
}

pub fn creator(tags: &BTreeMap<&str, bool>) -> Option<Box<Facet>> {
    if tags.contains_key("workspace.java.gradle") || tags.contains_key("workspace.java.pom") {
        let facet = JavaFacet {
            jvm: JvmFacet {
                is_gradle: tags.contains_key("workspace.java.gradle"),
                is_maven: tags.contains_key("workspace.java.pom"),
                has_java: false,
                has_groovy: false,
                has_kotlin: false,
                has_scala: false,
            },
            include_test: false,
        };
        return Some(Box::new(facet));
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::facet::JavaFacet;

    #[test]
    fn should_ident_test_java_file() {
        assert_eq!(false, JavaFacet::is_test("Hello.java"));
        assert_eq!(true, JavaFacet::is_test("HelloTest.java"));
        assert_eq!(true, JavaFacet::is_test("HelloTests.java"));
    }

    #[test]
    fn should_ident_maven_project() {
        assert_eq!(true, JavaFacet::is_maven("parent/model/pom.xml"));
        assert_eq!(true, JavaFacet::is_maven("parent/pom.xml"));
        assert_eq!(false, JavaFacet::is_maven("parent/Pom.xml"));
        assert_eq!(false, JavaFacet::is_maven("parent/model/Pom.xml"));
    }
}
