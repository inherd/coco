use crate::{Dependency, DependencyAnalyzer, DependencyScope};
use std::fs::read_to_string;
use std::path::PathBuf;
use sxd_document::parser;
use sxd_xpath::{Context, Factory, Value};

pub struct MavenDependencyAnalyzer {}

impl DependencyAnalyzer for MavenDependencyAnalyzer {
    fn is_build_file(&self, file: &str) -> bool {
        match file {
            "pom.xml" => true,
            _ => false,
        }
    }

    fn analysis_dependencies(&self, module_path: &str, _build_file: &str) -> Vec<Dependency> {
        let build_file_path = PathBuf::from(module_path)
            .join(_build_file)
            .display()
            .to_string();
        let build_file_content =
            read_to_string(build_file_path.as_str()).expect("can not open build file");
        match !build_file_content.is_empty() {
            true => parse_deps(build_file_content.as_str()),
            _ => vec![],
        }
    }
}

fn parse_deps(xml_content: &str) -> Vec<Dependency> {
    let mut deps = vec![];
    let project = parser::parse(xml_content).unwrap();
    let document = project.as_document();
    let factory = Factory::new();
    let mut context = Context::new();
    context.set_namespace("ns", "http://maven.apache.org/POM/4.0.0");

    let xpath = factory
        .build("count(/ns:project/ns:dependencies/ns:dependency)")
        .unwrap()
        .unwrap();
    let num_of_deps = xpath.evaluate(&context, document.root()).unwrap();
    if let Value::Number(ref count) = num_of_deps {
        for i in 0..(count.round() as i64) {
            let group_id_expression = format!(
                "/ns:project/ns:dependencies/ns:dependency[{}]/ns:groupId",
                i + 1
            );
            let group_id_xpath = factory
                .build(group_id_expression.as_str())
                .unwrap()
                .unwrap();
            let group = group_id_xpath
                .evaluate(&context, document.root())
                .unwrap()
                .string();

            let artifact_id_expression = format!(
                "/ns:project/ns:dependencies/ns:dependency[{}]/ns:artifactId",
                i + 1
            );
            let artifact_id_xpath = factory
                .build(artifact_id_expression.as_str())
                .unwrap()
                .unwrap();
            let name = artifact_id_xpath
                .evaluate(&context, document.root())
                .unwrap()
                .string();

            let version_expression = format!(
                "/ns:project/ns:dependencies/ns:dependency[{}]/ns:version",
                i + 1
            );
            let version_xpath = factory.build(version_expression.as_str()).unwrap().unwrap();
            let version = version_xpath
                .evaluate(&context, document.root())
                .unwrap()
                .string();

            let scope_expression = format!(
                "/ns:project/ns:dependencies/ns:dependency[{}]/ns:scope",
                i + 1
            );
            let scope_xpath = factory.build(scope_expression.as_str()).unwrap().unwrap();
            let scope_content = scope_xpath.evaluate(&context, document.root()).unwrap();
            let scope = match scope_content.string().as_str() {
                "test" => DependencyScope::Test,
                _ => DependencyScope::Compile,
            };

            deps.push(Dependency {
                group,
                name,
                version,
                scope,
            });
        }
    };
    deps
}
