use crate::{Dependency, DependencyAnalyzer, DependencyScope};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;
use sxd_document::{parser, Package};
use sxd_xpath::{Context, Factory, Value};

pub struct MavenDependencyAnalyzer {
    pub properties: HashMap<String, String>,
}

struct PomParser<'a> {
    factory: Factory,
    context: Context<'a>,
    package: Package,
}

impl<'a> PomParser<'a> {
    fn new(xml_content: &str) -> PomParser {
        let mut context = Context::new();
        context.set_namespace("ns", "http://maven.apache.org/POM/4.0.0");

        PomParser {
            package: parser::parse(xml_content).unwrap(),
            factory: Factory::new(),
            context,
        }
    }

    fn xpath_evaluate(&self, expression: &str) -> Value {
        let xpath = self.factory.build(expression).unwrap().unwrap();
        xpath
            .evaluate(&self.context, self.package.as_document().root())
            .unwrap()
    }

    fn parse_properties(&self) -> HashMap<String, String> {
        let mut properties = HashMap::with_capacity(10);
        let num_of_properties = self.xpath_evaluate("count(/ns:project/ns:properties/*)");
        if let Value::Number(ref count) = num_of_properties {
            for i in 0..(count.round() as i64) {
                let prop_name_expression = format!("name(/ns:project/ns:properties/*[{}])", i + 1);
                let prop_value_expression = format!("/ns:project/ns:properties/*[{}]", i + 1);

                let prop_name = self.xpath_evaluate(prop_name_expression.as_str());
                let prop_value = self.xpath_evaluate(prop_value_expression.as_str());
                properties.insert(prop_name.string(), prop_value.string());
            }
        }
        properties
    }

    fn parse_deps(&self) -> Vec<Dependency> {
        let mut deps = vec![];
        let num_of_deps = self.xpath_evaluate("count(/ns:project/ns:dependencies/ns:dependency)");
        if let Value::Number(ref count) = num_of_deps {
            for i in 0..(count.round() as i64) {
                deps.push(Dependency {
                    group: self.parse_group_id(i),
                    name: self.parse_artifact_id(i),
                    version: self.parse_version(i),
                    scope: self.parse_scope(i),
                });
            }
        };
        deps
    }

    fn parse_group_id(&self, index_of_dep: i64) -> String {
        let group_id_expression = format!(
            "/ns:project/ns:dependencies/ns:dependency[{}]/ns:groupId",
            index_of_dep + 1
        );
        self.xpath_evaluate(group_id_expression.as_str()).string()
    }

    fn parse_artifact_id(&self, index_of_dep: i64) -> String {
        let artifact_id_expression = format!(
            "/ns:project/ns:dependencies/ns:dependency[{}]/ns:artifactId",
            index_of_dep + 1
        );
        self.xpath_evaluate(artifact_id_expression.as_str())
            .string()
    }

    fn parse_version(&self, index_of_dep: i64) -> String {
        let version_expression = format!(
            "/ns:project/ns:dependencies/ns:dependency[{}]/ns:version",
            index_of_dep + 1
        );
        let version = self.xpath_evaluate(version_expression.as_str()).string();
        if version.starts_with("${") && version.ends_with("}") {
            let properties = self.parse_properties();
            let prop_name = &version[2..version.len() - 1];
            return properties.get(prop_name).unwrap().as_str().to_string();
        }
        version
    }

    fn parse_scope(&self, index_of_dep: i64) -> DependencyScope {
        let scope_expression = format!(
            "/ns:project/ns:dependencies/ns:dependency[{}]/ns:scope",
            index_of_dep + 1
        );
        let scope_content = self.xpath_evaluate(scope_expression.as_str());
        let scope = match scope_content.string().as_str() {
            "test" => DependencyScope::Test,
            _ => DependencyScope::Compile,
        };
        scope
    }
}

impl MavenDependencyAnalyzer {}

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
            true => {
                let pom_parser = PomParser::new(build_file_content.as_str());
                pom_parser.parse_deps()
            }
            _ => vec![],
        }
    }
}
