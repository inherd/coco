use crate::pas_content_root::ContentRoot;
use crate::psa_facet::Facet;
use crate::psa_library::Library;

pub struct Module {
    pub name: String,
    pub path: String,
    pub facets: Vec<Facet>,
    pub libraries: Vec<Library>,
    pub sub_modules: Vec<Module>,
    pub content_root: ContentRoot,
}

impl Module {
    pub fn add_facet(&mut self, facet: Facet) {
        self.facets.push(facet);
    }

    pub fn add_library(&mut self, lib: Library) {
        self.libraries.push(lib);
    }

    pub fn add_sub_module(&mut self, sub_module: Module) {
        self.sub_modules.push(sub_module);
    }

    pub fn new(name: &str, path: &str) -> Self {
        Module {
            name: name.to_string(),
            path: path.to_string(),
            facets: vec![],
            libraries: vec![],
            sub_modules: vec![],
            content_root: ContentRoot::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Facet, Library, LibraryScope, Module};

    #[test]
    fn should_create_module() {
        let module = Module::new("foo", "test/path");

        assert_eq!(module.name, "foo".to_string());
        assert_eq!(module.path, "test/path".to_string());
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

        module.add_library(Library {
            group: "org.springframework.boot".to_string(),
            name: "spring-boot-starter-web".to_string(),
            version: "1.0.0-RELEASE".to_string(),
            scope: LibraryScope::Compile,
        });

        let lib = module.libraries.get(0).unwrap();
        assert_eq!(module.libraries.len(), 1);
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
