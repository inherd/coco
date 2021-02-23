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
}

#[cfg(test)]
mod tests {
    use crate::psa_module::Module;
    use crate::psa_project::Project;

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

    #[test]
    fn should_add_modules() {
        let mut project = Project {
            name: "foo".to_string(),
            path: "test/path".to_string(),
            modules: vec![],
        };

        project.add_module(Module {
            name: "module1".to_string(),
            path: "test/path/module1".to_string(),
            facets: vec![],
        });

        project.add_module(Module {
            name: "module2".to_string(),
            path: "test/path/module2".to_string(),
            facets: vec![],
        });

        assert_eq!(project.modules.len(), 2);
    }
}
