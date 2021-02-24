use serde::{Deserialize, Serialize};

/// Coco Config from `coco.yml`
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CocoConfig {
    pub repos: Vec<RepoConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<Vec<CocoPlugin>>,
}

impl Default for CocoConfig {
    fn default() -> Self {
        CocoConfig {
            repos: vec![],
            plugins: None,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RepoConfig {
    pub url: String,
    pub languages: Option<Vec<String>>,
}

impl Default for RepoConfig {
    fn default() -> Self {
        RepoConfig {
            url: "".to_string(),
            languages: None,
        }
    }
}

impl RepoConfig {
    pub fn new(url: &str) -> RepoConfig {
        RepoConfig {
            url: url.to_string(),
            languages: None,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CocoPlugin {
    pub name: String,
    pub config: Option<Vec<CocoPluginConfig>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CocoPluginConfig {
    pub key: String,
    pub value: String,
}

#[cfg(test)]
mod test {
    use crate::CocoConfig;

    #[test]
    fn should_parse_language() {
        let data = r#"
repos:
  - url: https://github.com/projectfluent/fluent-rs
    languages: [Rust, JavaScript]

plugins:
  - name: swagger
  - name: struct_analysis
    config:
      - key: ctags
        value: /usr/local/bin/ctags
"#;
        let config: CocoConfig = serde_yaml::from_str(&data).expect("parse config file error");
        let repos = config.repos;
        let languages = repos[0].languages.as_ref().unwrap();
        assert_eq!(2, languages.len());
        assert_eq!("Rust", languages[0]);
    }
}
