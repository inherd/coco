extern crate xml;

use crate::{Dependency, DependencyAnalyzer, DependencyScope};

pub struct MavenDependencyAnalyzer {}

impl DependencyAnalyzer for MavenDependencyAnalyzer {
    fn is_build_file(&self, file: &str) -> bool {
        match file {
            "pom.xml" => true,
            _ => false,
        }
    }

    fn analysis_dependencies(&self, _build_file: &str) -> Vec<Dependency> {
        let mut dependencies = Vec::new();
        dependencies.push(Dependency {
            group: "org.springframework.boot".to_string(),
            name: "spring-boot-starter-web".to_string(),
            version: "2.0.0.RELEASE".to_string(),
            scope: DependencyScope::Test,
        });
        dependencies.push(Dependency {
            group: "org.springframework.boot".to_string(),
            name: "spring-boot-starter-logging".to_string(),
            version: "1.0.0.RELEASE".to_string(),
            scope: DependencyScope::Compile,
        });
        dependencies
    }
}
