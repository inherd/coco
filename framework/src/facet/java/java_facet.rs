use crate::facet::java::jvm_facet::JvmFacet;
use crate::facet::Facet;
use crate::lang::jvm;
use crate::lang::jvm::{WORKSPACE_FRAMEWORK_GRADLE, WORKSPACE_FRAMEWORK_POM};
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
                is_gradle: tags.contains_key(jvm::WORKSPACE_FRAMEWORK_GRADLE),
                is_maven: tags.contains_key(jvm::WORKSPACE_FRAMEWORK_POM),
                has_java: tags.contains_key(jvm::WORKSPACE_SOURCE_JAVA),
                has_groovy: tags.contains_key(jvm::WORKSPACE_SOURCE_GROOVY),
                has_kotlin: tags.contains_key(jvm::WORKSPACE_SOURCE_KOTLIN),
                has_scala: tags.contains_key(jvm::WORKSPACE_SOURCE_SCALA),
            },
            include_test: tags.contains_key(jvm::WORKSPACE_HAS_TEST),
        };
        return Some(Box::new(facet));
    }
    None
}

fn is_jvm_project(tags: &BTreeMap<&str, bool>) -> bool {
    tags.contains_key(WORKSPACE_FRAMEWORK_GRADLE) || tags.contains_key(WORKSPACE_FRAMEWORK_POM)
}
