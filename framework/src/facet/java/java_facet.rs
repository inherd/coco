use crate::facet::java::jvm_facet::JvmFacet;
use regex::Regex;

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
