use std::collections::HashMap;

use regex::{Match, Regex};

use crate::domain::git::{CocoCommit, GitFileChange};

lazy_static! {
    static ref REV: Regex = Regex::new(r"\[(?P<rev>[\d|a-f]{5,12})\]").unwrap();
    static ref AUTHOR: Regex = Regex::new(r"(.*?)\s\d{4}-\d{2}-\d{2}").unwrap();
    static ref DATE: Regex = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();
    static ref CHANGES: Regex = Regex::new(r"([\d-]+)[\t\s]+([\d-]+)[\t\s]+(.*)").unwrap();
    static ref COMPLEXMOVEREGSTR: Regex = Regex::new(r"(.*)\{(.*)\s=>\s(.*)\}(.*)").unwrap();
    static ref BASICMOVEREGSTR: Regex = Regex::new(r"(.*)\s=>\s(.*)").unwrap();
    static ref CHANGEMODEL: Regex =
        Regex::new(r"\s(\w{1,6})\s(mode 100(\d){3})?\s?(.*)(\s\(\d{2}%\))?").unwrap();
}

pub struct GitMessageParser {
    current_commit: CocoCommit,
    current_file_change: Vec<GitFileChange>,
    commits: Vec<CocoCommit>,
    current_file_change_map: HashMap<String, GitFileChange>,
}

impl Default for GitMessageParser {
    fn default() -> Self {
        GitMessageParser {
            current_commit: Default::default(),
            current_file_change: vec![],
            commits: vec![],
            current_file_change_map: Default::default(),
        }
    }
}

impl GitMessageParser {
    pub fn to_commit_message(str: &str) -> Vec<CocoCommit> {
        let split = str.split("\n");
        let mut parser = GitMessageParser::default();
        for line in split {
            parser.parse_log_by_line(line)
        }

        parser.commits
    }

    pub fn parse_log_by_line(&mut self, str: &str) {
        let changeMode = "";
        let find_rev = REV.captures(str);
        if let Some(captures) = find_rev {
            let rev = (&captures["rev"]).to_string();
            self.current_commit = CocoCommit {
                branch: "".to_string(),
                rev: rev,
                author: "".to_string(),
                committer: "".to_string(),
                date: 0,
                message: "".to_string(),
                changes: vec![],
            }
        } else if let Some(caps) = CHANGES.captures(str) {
            // todo
        } else if let Some(caps) = CHANGEMODEL.captures(str) {
            // todo
        } else if self.current_commit.rev != "" {
            self.commits.push(self.current_commit.clone());
        }
    }
}

#[cfg(test)]
mod test {
    use crate::infrastructure::git::git_message_parser::GitMessageParser;

    #[test]
    pub fn should_success_parse_one_line_log() {
        let input = "[828fe39523] Rossen Stoyanchev 2019-12-04 Consistently use releaseBody in DefaultWebClient
5       3       spring-webflux/core/main/java/org/springframework/web/reactive/function/client/ClientResponse.java
1       1       spring-webflux/core/main/java/org/springframework/web/reactive/function/client/DefaultWebClient.java
9       3       spring-webflux/core/main/java/org/springframework/web/reactive/function/client/WebClient.java
6       11      core/docs/asciidoc/web/webflux-webclient.adoc
";

        let commits = GitMessageParser::to_commit_message(input);
        assert_eq!(1, commits.len());
        assert_eq!("828fe39523", commits[0].rev);
    }
}
