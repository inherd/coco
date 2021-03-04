use crate::pas_content_root::ContentRoot;
use crate::psa_dependency::Dependency;
use crate::psa_facet::Facet;

#[derive(Serialize)]
pub struct Module {
    pub name: String,
    pub relative_path: String,
    pub facets: Vec<Facet>,
    pub dependencies: Vec<Dependency>,
    pub sub_modules: Vec<Module>,
    pub content_root: ContentRoot,
}

impl Module {
    pub fn add_facet(&mut self, facet: Facet) {
        self.facets.push(facet);
    }

    pub fn add_library(&mut self, lib: Dependency) {
        self.dependencies.push(lib);
    }

    pub fn add_sub_module(&mut self, sub_module: Module) {
        self.sub_modules.push(sub_module);
    }

    pub fn add_sub_modules(&mut self, sub_modules: &mut Vec<Module>) {
        self.sub_modules.append(sub_modules);
    }

    pub fn add_source_root(&mut self, source_root: String) {
        self.content_root.add_source_root(source_root.as_str());
    }

    pub fn add_resource_root(&mut self, resource_root: String) {
        self.content_root.add_resource_root(resource_root.as_str());
    }

    pub fn add_test_source_root(&mut self, test_source_root: String) {
        self.content_root
            .add_test_source_root(test_source_root.as_str());
    }

    pub fn add_test_resource_root(&mut self, test_resource_root: String) {
        self.content_root
            .add_test_resource_root(test_resource_root.as_str());
    }

    pub fn set_content_root(&mut self, content_root: ContentRoot) {
        self.content_root = content_root;
    }

    pub fn add_dependencies(&mut self, dependencies: &mut Vec<Dependency>) {
        self.dependencies.append(dependencies);
    }

    pub fn new(name: &str, path: &str) -> Self {
        Module {
            name: name.to_string(),
            relative_path: path.to_string(),
            facets: vec![],
            dependencies: vec![],
            sub_modules: vec![],
            content_root: ContentRoot::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Dependency, DependencyScope, Facet, Module};

    #[test]
    fn should_create_module() {
        let module = Module::new("foo", "test/path");

        assert_eq!(module.name, "foo".to_string());
        assert_eq!(module.relative_path, "test/path".to_string());
    }

    #[test]
    fn should_add_facet() {
        let mut module = Module::new("foo", "test/path");

        module.add_facet(Facet {
            name: "Java".to_string(),
        });

        assert_eq!(module.facets.len(), 1);
    }

    #[test]
    fn should_add_library() {
        let mut module = Module::new("foo", "test/path");

        module.add_library(Dependency {
            group: "org.springframework.boot".to_string(),
            name: "spring-boot-starter-web".to_string(),
            version: "1.0.0-RELEASE".to_string(),
            scope: DependencyScope::Compile,
        });

        let lib = module.dependencies.get(0).unwrap();
        assert_eq!(module.dependencies.len(), 1);
        assert_eq!(lib.name, "spring-boot-starter-web");
    }

    #[test]
    fn should_add_sub_module() {
        let mut module = Module::new("parent", "test/path");

        module.add_sub_module(Module::new("child", "test/child/path"));

        let sub_module = module.sub_modules.get(0).unwrap();
        assert_eq!(module.sub_modules.len(), 1);
        assert_eq!(sub_module.name, "child")
    }
}
