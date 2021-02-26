use crate::psa_module::Module;

#[derive(Serialize)]
pub struct Project {
    pub name: String,
    pub path: String,
    pub modules: Vec<Module>,
    pub project_type: String,
}

impl Project {
    pub fn add_module(&mut self, module: Module) {
        self.modules.push(module);
    }

    pub fn add_modules(&mut self, modules: &mut Vec<Module>) {
        self.modules.append(modules)
    }

    pub fn new(name: &str, path: &str, project_type: &str) -> Self {
        Project {
            name: name.to_string(),
            path: path.to_string(),
            modules: vec![],
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
        assert_eq!(project.path, "test/path".to_string());
        assert_eq!(project.project_type, "maven".to_string());
        assert_eq!(project.modules.is_empty(), true);
    }

    #[test]
    fn should_add_modules() {
        let mut project = Project::new("foo", "test/path", "maven");

        project.add_module(Module::new("module1", "test/path/module1"));
        project.add_module(Module::new("module2", "test/path/module2"));

        assert_eq!(project.modules.len(), 2);
    }
}
