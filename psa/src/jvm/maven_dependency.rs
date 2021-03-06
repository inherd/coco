use crate::{Dependency, DependencyAnalyzer, DependencyScope};
use std::fs::read_to_string;
use std::path::PathBuf;
use sxd_document::{parser, Package};
use sxd_xpath::{Context, Factory, Value};

pub struct MavenDependencyAnalyzer {}

struct XPathContext<'a> {
    factory: Factory,
    context: Context<'a>,
    package: Package,
}

impl<'a> XPathContext<'a> {
    fn new(xml_content: &str) -> XPathContext {
        let mut context = Context::new();
        context.set_namespace("ns", "http://maven.apache.org/POM/4.0.0");
        XPathContext {
            package: parser::parse(xml_content).unwrap(),
            factory: Factory::new(),
            context,
        }
    }

    fn evaluate(&self, expression: &str) -> Value {
        let xpath = self.factory.build(expression).unwrap().unwrap();
        xpath
            .evaluate(&self.context, self.package.as_document().root())
            .unwrap()
    }
}

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
    let xpath_context = XPathContext::new(xml_content);

    let num_of_deps = xpath_context.evaluate("count(/ns:project/ns:dependencies/ns:dependency)");
    if let Value::Number(ref count) = num_of_deps {
        for i in 0..(count.round() as i64) {
            deps.push(Dependency {
                group: parse_group_id(&xpath_context, i),
                name: parse_artifact_id(&xpath_context, i),
                version: parse_version(&xpath_context, i),
                scope: parse_scope(&xpath_context, i),
            });
        }
    };
    deps
}

fn parse_group_id(xpath_context: &XPathContext, i: i64) -> String {
    let group_id_expression = format!(
        "/ns:project/ns:dependencies/ns:dependency[{}]/ns:groupId",
        i + 1
    );
    xpath_context
        .evaluate(group_id_expression.as_str())
        .string()
}

fn parse_artifact_id(xpath_context: &XPathContext, i: i64) -> String {
    let artifact_id_expression = format!(
        "/ns:project/ns:dependencies/ns:dependency[{}]/ns:artifactId",
        i + 1
    );
    let name = xpath_context
        .evaluate(artifact_id_expression.as_str())
        .string();
    name
}

fn parse_version(xpath_context: &XPathContext, i: i64) -> String {
    let version_expression = format!(
        "/ns:project/ns:dependencies/ns:dependency[{}]/ns:version",
        i + 1
    );
    let version = xpath_context.evaluate(version_expression.as_str()).string();
    version
}

fn parse_scope(xpath_context: &XPathContext, i: i64) -> DependencyScope {
    let scope_expression = format!(
        "/ns:project/ns:dependencies/ns:dependency[{}]/ns:scope",
        i + 1
    );
    let scope_content = xpath_context.evaluate(scope_expression.as_str());
    let scope = match scope_content.string().as_str() {
        "test" => DependencyScope::Test,
        _ => DependencyScope::Compile,
    };
    scope
}
