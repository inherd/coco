use crate::psa_facet::Facet;

pub struct Module {
    pub name: String,
    pub path: String,
    pub facets: Vec<Facet>,
}

impl Module {
    pub fn add_facet(&mut self, facet: Facet) {
        self.facets.push(facet);
    }
}

#[cfg(test)]
mod tests {
    use crate::psa_facet::Facet;
    use crate::psa_module::Module;

    #[test]
    fn should_create_module() {
        let module = Module {
            name: "foo".to_string(),
            path: "test/path".to_string(),
            facets: vec![],
        };

        assert_eq!(module.name, "foo".to_string());
        assert_eq!(module.path, "test/path".to_string());
    }

    #[test]
    fn should_add_facet() {
        let mut module = Module {
            name: "foo".to_string(),
            path: "test/path".to_string(),
            facets: vec![],
        };

        module.add_facet(Facet {
            name: "Java".to_string(),
        });

        assert_eq!(module.facets.len(), 1);
    }
}
