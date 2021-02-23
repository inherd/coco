use crate::pas_content_root::ContentRoot;
use crate::psa_facet::Facet;
use crate::psa_library::Library;

pub struct Module {
    pub name: String,
    pub path: String,
    pub facets: Vec<Facet>,
    pub libraries: Vec<Library>,
    pub content_root: ContentRoot,
}

impl Module {
    pub fn add_facet(&mut self, facet: Facet) {
        self.facets.push(facet);
    }

    pub fn add_library(&mut self, lib: Library) {
        self.libraries.push(lib);
    }

    pub fn new(name: &str, path: &str) -> Self {
        Module {
            name: name.to_string(),
            path: path.to_string(),
            facets: vec![],
            libraries: vec![],
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
        let lib = Library {
            group: "org.springframework.boot".to_string(),
            name: "spring-boot-starter-web".to_string(),
            version: "1.0.0-RELEASE".to_string(),
            scope: LibraryScope::Compile,
        };

        module.add_library(lib);

        assert_eq!(module.libraries.len(), 1);
    }
}
