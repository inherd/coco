#[derive(Serialize)]
pub struct ContentRoot {
    pub source_root: Vec<String>,
    pub resource_root: Vec<String>,
    pub test_source_root: Vec<String>,
    pub test_resource_root: Vec<String>,
}

impl ContentRoot {
    pub fn add_source_root(&mut self, root_path: &str) {
        self.source_root.push(root_path.to_string());
    }

    pub fn add_resource_root(&mut self, root_path: &str) {
        self.resource_root.push(root_path.to_string());
    }

    pub fn add_test_source_root(&mut self, root_path: &str) {
        self.test_source_root.push(root_path.to_string());
    }

    pub fn add_test_resource_root(&mut self, root_path: &str) {
        self.test_resource_root.push(root_path.to_string());
    }
}

impl Default for ContentRoot {
    fn default() -> Self {
        ContentRoot {
            source_root: vec![],
            resource_root: vec![],
            test_source_root: vec![],
            test_resource_root: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ContentRoot;

    #[test]
    fn should_add_various_roots() {
        let mut content_root = ContentRoot::default();

        content_root.add_source_root("src/main/java");
        content_root.add_resource_root("src/main/resources");
        content_root.add_test_source_root("src/test/java");
        content_root.add_test_resource_root("src/test/resources");

        assert_eq!(content_root.source_root, vec!["src/main/java".to_string()]);
        assert_eq!(
            content_root.resource_root,
            vec!["src/main/resources".to_string()]
        );
        assert_eq!(
            content_root.test_source_root,
            vec!["src/test/java".to_string()]
        );
        assert_eq!(
            content_root.test_resource_root,
            vec!["src/test/resources".to_string()]
        );
    }
}
