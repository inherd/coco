use serde::{Deserialize, Serialize};

/// Coco Config from `coco.yml`
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CocoConfig {
    pub repo: Vec<RepoConfig>,
    pub plugins: Vec<String>,
}

impl Default for CocoConfig {
    fn default() -> Self {
        CocoConfig {
            repo: vec![],
            plugins: vec![],
        }
    }
}

/// RepoConfig
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RepoConfig {
    pub url: String,
    pub language: Option<Vec<String>>,
}

impl Default for RepoConfig {
    fn default() -> Self {
        RepoConfig {
            url: "".to_string(),
            language: None,
        }
    }
}

impl RepoConfig {
    pub fn new(url: &str) -> RepoConfig {
        RepoConfig {
            url: url.to_string(),
            language: None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::CocoConfig;

    #[test]
    fn should_parse_language() {
        let data = r#"
repo:
  - url: https://github.com/projectfluent/fluent-rs
    language: [Rust, JavaScript]

plugins:
  - swagger
"#;
        let config: CocoConfig = serde_yaml::from_str(&data).expect("parse config file error");
        let repos = config.repo;
        let languages = repos[0].language.as_ref().unwrap();
        assert_eq!(2, languages.len());
        assert_eq!("Rust", languages[0]);
    }
}
