use crate::psa_module::Module;

#[derive(Serialize)]
pub struct Project {
    pub name: String,
    pub absolute_path: String,
    pub project_module: Option<Module>,
    pub project_type: String,
}

impl Project {
    pub fn set_project_module(&mut self, module: Module) {
        self.project_module = Some(module);
    }

    pub fn new(name: &str, path: &str, project_type: &str) -> Self {
        Project {
            name: name.to_string(),
            absolute_path: path.to_string(),
            project_module: None,
            project_type: project_type.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Module, Project};

    #[test]
    fn should_create_project() {
        let project = Project::new("foo", "test/path", "maven");

        assert_eq!(project.name, "foo".to_string());
        assert_eq!(project.absolute_path, "test/path".to_string());
        assert_eq!(project.project_type, "maven".to_string());
        assert_eq!(project.project_module.is_none(), true);
    }

    #[test]
    fn should_add_modules() {
        let mut project = Project::new("foo", "test/path", "maven");

        project.set_project_module(Module::new("module1", "test/path/module1"));
        project.set_project_module(Module::new("module2", "test/path/module2"));

        assert_eq!(project.project_module.is_none(), false);
    }
}
