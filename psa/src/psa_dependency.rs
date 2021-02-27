#[derive(Serialize)]
pub struct Dependency {
    pub name: String,
    pub group: String,
    pub version: String,
    pub scope: DependencyScope,
}

#[derive(Serialize, PartialEq, Debug)]
pub enum DependencyScope {
    Test,
    Compile,
}

#[cfg(test)]
mod tests {
    use crate::{Dependency, DependencyScope};

    #[test]
    fn should_create_dependency() {
        let lib = Dependency {
            group: "org.springframework.boot".to_string(),
            name: "spring-boot-starter-web".to_string(),
            version: "1.0.0-RELEASE".to_string(),
            scope: DependencyScope::Compile,
        };

        assert_eq!(lib.group, "org.springframework.boot".to_string());
        assert_eq!(lib.name, "spring-boot-starter-web".to_string());
        assert_eq!(lib.version, "1.0.0-RELEASE".to_string());
        assert_eq!(lib.scope, DependencyScope::Compile);
    }
}
