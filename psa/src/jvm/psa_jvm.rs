use std::path::Path;

use crate::jvm::maven_module::MavenModuleAnalyzer;
use crate::{files, ModuleAnalyzer, ProjectStructureAnalyzer};

pub struct JvmProjectStructureAnalyzer {}

impl Default for JvmProjectStructureAnalyzer {
    fn default() -> Self {
        JvmProjectStructureAnalyzer {}
    }
}

impl ProjectStructureAnalyzer for JvmProjectStructureAnalyzer {
    fn get_project_name(&self, project_path: &str) -> String {
        Path::new(project_path)
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap()
    }

    fn get_project_type(&self) -> String {
        "maven".to_string()
    }

    fn is_related(&self, project_path: &str) -> bool {
        let files = files::list_file_names(project_path);

        for file_name in files.iter() {
            if is_build_file(file_name) {
                return true;
            }
        }

        false
    }

    fn get_module_analyzers(&self) -> Vec<Box<dyn ModuleAnalyzer>> {
        vec![Box::new(MavenModuleAnalyzer {})]
    }
}

fn is_build_file(file_name: &str) -> bool {
    match file_name {
        "pom.xml" => true,
        "build.gradle" => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::files::join_path;
    use crate::jvm::psa_jvm::JvmProjectStructureAnalyzer;
    use crate::{DependencyScope, Project, ProjectStructureAnalyzer};

    #[test]
    fn should_analysis_maven_project_sub_modules() {
        let project = do_analysis(vec![
            "_fixtures",
            "projects",
            "java",
            "multi_mod_maven_project",
        ]);

        let project_module = project.project_module.unwrap();
        let module1 = project_module.sub_modules.get(0).unwrap();
        let module2 = project_module.sub_modules.get(1).unwrap();
        assert_eq!(project_module.sub_modules.len(), 2);
        assert_eq!(project.project_type, "maven");
        assert_eq!(project_module.name, "multi_mod_maven_project");
        assert_eq!(module1.name, "module1");
        assert_eq!(module2.name, "module2");
    }

    #[test]
    fn should_detect_project_module_content_root() {
        let project = do_analysis(vec![
            "_fixtures",
            "projects",
            "java",
            "multi_mod_maven_project",
        ]);
        let project_module = project.project_module.unwrap();
        let project_content_root = &project_module.content_root;

        let expect_source_path = join_path("", vec!["src", "main", "java"]);
        assert_eq!(project_content_root.source_root.len(), 1);
        assert_eq!(
            project_content_root.source_root.get(0).unwrap().as_str(),
            expect_source_path.as_str()
        );

        let expect_resource_path = join_path("", vec!["src", "main", "resources"]);
        assert_eq!(project_content_root.resource_root.len(), 1);
        assert_eq!(
            project_content_root.resource_root.get(0).unwrap().as_str(),
            expect_resource_path.as_str()
        );

        let expect_test_source_root = join_path("", vec!["src", "test", "java"]);
        assert_eq!(project_content_root.test_source_root.len(), 1);
        assert_eq!(
            project_content_root
                .test_source_root
                .get(0)
                .unwrap()
                .as_str(),
            expect_test_source_root.as_str()
        );

        let expect_test_resources_root = join_path("", vec!["src", "test", "resources"]);
        assert_eq!(project_content_root.test_resource_root.len(), 1);
        assert_eq!(
            project_content_root.test_resource_root.get(0).unwrap(),
            expect_test_resources_root.as_str()
        );
    }

    #[test]
    fn should_detect_sub_module_content_root() {
        let project = do_analysis(vec![
            "_fixtures",
            "projects",
            "java",
            "multi_mod_maven_project",
        ]);

        let project_module = project.project_module.unwrap();
        let module1 = project_module.sub_modules.get(0).unwrap();
        let content_root = &module1.content_root;

        let expect_source_path = join_path("", vec!["src", "main", "java"]);
        assert_eq!(
            content_root.source_root.get(0).unwrap().as_str(),
            expect_source_path
        );

        let expect_test_source_root = join_path("", vec!["src", "test", "java"]);
        assert_eq!(
            content_root.test_source_root.get(0).unwrap().as_str(),
            expect_test_source_root.as_str()
        );

        let expect_test_source_root = join_path("", vec!["src", "test", "java"]);
        assert_eq!(
            content_root.test_source_root.get(0).unwrap().as_str(),
            expect_test_source_root.as_str()
        );

        let expect_test_resources_root = join_path("", vec!["src", "test", "resources"]);
        assert_eq!(
            content_root.test_resource_root.get(0).unwrap(),
            expect_test_resources_root.as_str()
        );
    }

    #[test]
    fn should_analysis_dependencies() {
        let project = do_analysis(vec![
            "_fixtures",
            "projects",
            "java",
            "multi_mod_maven_project",
        ]);

        let project_module = project.project_module;
        let project_dependencies = project_module.unwrap().dependencies;

        assert_eq!(project_dependencies.len(), 2);

        let dep1 = project_dependencies.get(0).unwrap();
        assert_eq!(dep1.name, "spring-boot-starter-web");
        assert_eq!(dep1.group, "org.springframework.boot");
        assert_eq!(dep1.version, "2.0.0.RELEASE");
        assert_eq!(dep1.scope, DependencyScope::Test);

        let dep2 = project_dependencies.get(1).unwrap();
        assert_eq!(dep2.name, "spring-boot-starter-logging");
        assert_eq!(dep2.group, "org.springframework.boot");
        assert_eq!(dep2.version, "${spring-boot-starter.version}");
        assert_eq!(dep2.scope, DependencyScope::Compile);
    }

    fn do_analysis(path: Vec<&str>) -> Project {
        let mut project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();

        for path in path.into_iter() {
            project_dir.push(path);
        }

        let analyzer = JvmProjectStructureAnalyzer::default();
        analyzer.analysis(project_dir.display().to_string().as_str())
    }
}
