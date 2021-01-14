use crate::facet::jvm_facet::JvmFacet;
use regex::Regex;

lazy_static! {
    static ref JAVA_TEST: Regex = Regex::new(r".*(Tests|Test).java").unwrap();
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
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
}
