use crate::module::Module;

pub struct Project {
    pub name: String,
    pub path: String,
    pub modules: Vec<Module>,
}

#[cfg(test)]
mod tests {
    use crate::project::Project;

    #[test]
    fn should_create_project() {
        let project = Project {
            name: "foo".to_string(),
            path: "test/path".to_string(),
            modules: vec![],
        };

        assert_eq!(project.name, "foo".to_string());
        assert_eq!(project.path, "test/path".to_string());
        assert_eq!(project.modules.is_empty(), true);
    }
}
