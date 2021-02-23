use crate::psa_module::Module;

pub struct Project {
    pub name: String,
    pub path: String,
    pub modules: Vec<Module>,
}

impl Project {
    pub fn add_module(&mut self, module: Module) {
        self.modules.push(module);
    }

    pub fn new(name: &str, path: &str) -> Self {
        Project {
            name: name.to_string(),
            path: path.to_string(),
            modules: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Module, Project};

    #[test]
    fn should_create_project() {
        let project = Project::new("foo", "test/path");

        assert_eq!(project.name, "foo".to_string());
        assert_eq!(project.path, "test/path".to_string());
        assert_eq!(project.modules.is_empty(), true);
    }

    #[test]
    fn should_add_modules() {
        let mut project = Project::new("foo", "test/path");

        project.add_module(Module::new("module1", "test/path/module1"));
        project.add_module(Module::new("module2", "test/path/module2"));

        assert_eq!(project.modules.len(), 2);
    }
}
