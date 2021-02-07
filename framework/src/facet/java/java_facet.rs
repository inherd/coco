use crate::facet::java::jvm_facet::JvmFacet;
use crate::facet::Facet;
use std::collections::BTreeMap;

#[derive(Serialize)]
pub struct JavaFacet {
    pub jvm: JvmFacet,
    pub include_test: bool,
}

impl JavaFacet {
    pub fn new() -> JavaFacet {
        JavaFacet {
            jvm: Default::default(),
            include_test: false,
        }
    }
}

pub fn creator(tags: &BTreeMap<&str, bool>) -> Option<Box<Facet>> {
    if is_jvm_project(tags) {
        let facet = JavaFacet {
            jvm: JvmFacet {
                is_gradle: tags.contains_key("workspace.gradle"),
                is_maven: tags.contains_key("workspace.pom"),
                has_java: tags.contains_key("workspace.source.java"),
                has_groovy: tags.contains_key("workspace.source.groovy"),
                has_kotlin: tags.contains_key("workspace.source.kotlin"),
                has_scala: tags.contains_key("workspace.source.scala"),
            },
            include_test: tags.contains_key("workspace.source.test"),
        };
        return Some(Box::new(facet));
    }
    None
}

fn is_jvm_project(tags: &BTreeMap<&str, bool>) -> bool {
    tags.contains_key("workspace.gradle") || tags.contains_key("workspace.pom")
}
