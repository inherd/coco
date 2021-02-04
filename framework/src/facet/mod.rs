use crate::facet::java::java_facet;
pub use go::go_facet;
pub use java::jvm_facet::JvmFacet;
pub use java::JavaFacet;
pub use java::JavaModuleData;
pub use javascript::javascript_facet;
pub use python::python_facet;
pub use rust::rust_facet;
use std::collections::BTreeMap;

/// Java
pub mod java;

/// JavaScript
pub mod javascript;

/// Python
pub mod python;

/// golang
pub mod go;

/// rust
pub mod rust;

pub type Facet = dyn erased_serde::Serialize;
pub type FacetCreator = fn(&BTreeMap<&str, bool>) -> Option<Box<Facet>>;

pub struct FacetsBuilder {
    facets: Vec<FacetCreator>,
}

impl Default for FacetsBuilder {
    fn default() -> Self {
        FacetsBuilder {
            facets: vec![java_facet::creator],
        }
    }
}

impl FacetsBuilder {
    pub fn build(self, tags: &BTreeMap<&str, bool>) -> Vec<Box<Facet>> {
        let mut facets = Vec::new();
        for creator in self.facets.iter() {
            match creator(tags) {
                Some(facet) => facets.push(facet),
                _ => continue,
            }
        }
        facets
    }
}
