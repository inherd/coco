use serde::{Deserialize, Serialize};
use std::{env, fs};

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

impl CocoConfig {
    pub fn get_plugin_config(&self, plugin_name: &str) -> Option<Vec<CocoPluginConfig>> {
        if self.plugins.is_none() {
            return None;
        }
        let mut plugin = CocoPlugin::default();
        for item in self.plugins.as_ref().unwrap() {
            if item.name == plugin_name {
                plugin = item.clone();
            }
        }

        if plugin.name == "" {
            return None;
        }

        if plugin.config.is_none() {
            return None;
        }

        return Some(plugin.config.unwrap());
    }

    pub fn from_file(config_file: &str) -> CocoConfig {
        match fs::read_to_string(config_file) {
            Ok(content) => serde_yaml::from_str(&content).expect("parse config file error"),
            Err(_) => {
                let mut repo = vec![];
                let current = env::current_dir().unwrap();
                repo.push(RepoConfig {
                    url: current.into_os_string().to_str().unwrap().to_string(),
                    languages: None,
                });
                CocoConfig {
                    repos: repo,
                    plugins: None,
                }
            }
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

impl Default for CocoPlugin {
    fn default() -> Self {
        CocoPlugin {
            name: "".to_string(),
            config: None,
        }
    }
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

    #[test]
    fn should_enable_get_plugin_config() {
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
        let config = config.get_plugin_config("struct_analysis").unwrap();
        assert_eq!(1, config.len());
        assert_eq!("ctags", config[0].key);
        assert_eq!("/usr/local/bin/ctags", config[0].value);
    }
}
