use regex::Regex;

lazy_static! {
    static ref PYTHON_TEST: Regex = Regex::new(r"^test.*\.py").unwrap();
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PythonFacet {
    pub has_requirements: bool,
    pub include_test: bool,
}

impl PythonFacet {
    pub fn is_test(path: &str) -> bool {
        PYTHON_TEST.is_match(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_ident_pytest_file() {
        assert_eq!(true, PythonFacet::is_test("test_hello.py"));
        assert_eq!(false, PythonFacet::is_test("hello.py"));
        assert_eq!(false, PythonFacet::is_test("testhellopy"));
        assert_eq!(false, PythonFacet::is_test("fatest.py"));
    }
}
