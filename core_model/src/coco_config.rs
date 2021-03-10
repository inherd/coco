use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{env, fs};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CocoConfig {
    pub repos: Vec<RepoConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<Vec<CocoPlugin>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_config: Option<Vec<CocoCommitConfig>>,
}

/// Coco Commit Config from `coco.yml`
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CocoCommitConfig {
    pub regex: String,
    pub matches: Vec<String>,
    pub samples: Option<String>,
}

#[allow(dead_code)]
impl CocoCommitConfig {
    fn verify_config(config: &CocoCommitConfig) -> HashMap<String, String> {
        let mut items: HashMap<String, String> = Default::default();
        match Regex::new(&config.regex) {
            Ok(re) => match re.captures(&config.samples.as_ref().unwrap()) {
                None => {
                    println!("....");
                }
                Some(caps) => {
                    if caps.len() - 1 != config.matches.len() {
                        panic!(
                            "error, matches fields length {:?} not equal regex captures length {:?}",
                            caps.len() - 1,
                            config.matches.len()
                        );
                    }

                    let mut index = 1;
                    for key in &config.matches {
                        items.insert(key.clone(), caps.get(index).unwrap().as_str().to_string());
                        index = index + 1;
                    }
                }
            },
            Err(err) => {
                println!("parse regex error: {:?}", err);
            }
        }

        items
    }
}

impl Default for CocoConfig {
    fn default() -> Self {
        CocoConfig {
            repos: vec![],
            plugins: None,
            commit_config: None,
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

        if plugin.name.is_empty() {
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
                    commit_config: None,
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
    use crate::coco_config::CocoCommitConfig;
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

    #[test]
    fn should_match_commit_message_config() {
        let data = r#"
regex: ^(feature|fix)/([a-z,A-Z]+-\d*):(.*)
matches:
 - scope
 - id
 - message
samples: feature/JIR-124:test commit message
"#;

        let config: CocoCommitConfig =
            serde_yaml::from_str(&data).expect("parse config file error");

        let items = CocoCommitConfig::verify_config(&config);

        assert_eq!(3, items.len());
        assert_eq!("feature", items.get("scope").unwrap());
        assert_eq!("JIR-124", items.get("id").unwrap());
    }
}
