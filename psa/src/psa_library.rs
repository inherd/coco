#[derive(Serialize)]
pub struct Library {
    pub name: String,
    pub group: String,
    pub version: String,
    pub scope: LibraryScope,
}

#[derive(Serialize, PartialEq, Debug)]
pub enum LibraryScope {
    Test,
    Compile,
}

#[cfg(test)]
mod tests {
    use crate::{Library, LibraryScope};

    #[test]
    fn should_create_library() {
        let lib = Library {
            group: "org.springframework.boot".to_string(),
            name: "spring-boot-starter-web".to_string(),
            version: "1.0.0-RELEASE".to_string(),
            scope: LibraryScope::Compile,
        };

        assert_eq!(lib.group, "org.springframework.boot".to_string());
        assert_eq!(lib.name, "spring-boot-starter-web".to_string());
        assert_eq!(lib.version, "1.0.0-RELEASE".to_string());
        assert_eq!(lib.scope, LibraryScope::Compile);
    }
}
