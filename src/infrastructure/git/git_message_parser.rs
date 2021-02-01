use std::collections::HashMap;

use regex::{Captures, Regex};

use crate::domain::git::coco_commit::FileChange;
use crate::domain::git::CocoCommit;

lazy_static! {
    static ref COMMIT_ID: Regex = Regex::new(r"\[(?P<commit_id>[\d|a-f]{5,12})\]").unwrap();
    static ref AUTHOR: Regex = Regex::new(r"(?P<author>.*?)\s\d{10}").unwrap();
    static ref DATE: Regex = Regex::new(r"(?P<date>\d{10})").unwrap();
    static ref CHANGES: Regex =
        Regex::new(r"(?P<deleted>[\d-]+)[\t\s]+(?P<added>[\d-]+)[\t\s]+(?P<filename>.*)").unwrap();
    static ref COMPLEXMOVEREGSTR: Regex = Regex::new(r"(.*)\{(.*)\s=>\s(.*)\}(.*)").unwrap();
    static ref BASICMOVEREGSTR: Regex = Regex::new(r"(.*)\s=>\s(.*)").unwrap();
    static ref CHANGEMODEL: Regex =
        Regex::new(r"\s(\w{1,6})\s(mode 100(\d){3})?\s?(.*)(\s\(\d{2}%\))?").unwrap();
}

pub struct GitMessageParser {
    current_commit: CocoCommit,
    current_file_change: Vec<FileChange>,
    commits: Vec<CocoCommit>,
    current_file_change_map: HashMap<String, FileChange>,
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
    pub fn parse(str: &str) -> Vec<CocoCommit> {
        let split = str.split("\n");
        let mut parser = GitMessageParser::default();
        for line in split {
            parser.parse_log_by_line(line)
        }

        parser.commits
    }

    pub fn parse_log_by_line(&mut self, text: &str) {
        // COMMIT_ID -> CHANGES -> CHANGE_MODEL -> Push to Commits
        if let Some(captures) = COMMIT_ID.captures(text) {
            self.current_commit = GitMessageParser::create_commit(text, &captures)
        } else if let Some(caps) = CHANGES.captures(text) {
            let filename = caps["filename"].to_string();
            let file_change = GitMessageParser::create_file_change(filename.clone(), caps);
            self.current_file_change_map.insert(filename, file_change);
        } else if let Some(caps) = CHANGEMODEL.captures(text) {
            self.update_change_mode(caps)
        } else if self.current_commit.commit_id != "" {
            self.push_to_commits();
        }
    }

    fn push_to_commits(&mut self) {
        for (_filename, change) in &self.current_file_change_map {
            self.current_file_change.push(change.clone());
        }

        self.current_commit.changes = self.current_file_change.clone();
        self.commits.push(self.current_commit.clone());

        self.current_file_change_map.clear();
    }

    fn update_change_mode(&mut self, caps: Captures) {
        let change_model_index = 4;
        if caps.len() > change_model_index {
            let mode = caps.get(1).unwrap().as_str();
            let file_name = caps.get(4).unwrap().as_str();
            if self.current_file_change_map.get(file_name).is_some() {
                let change = self.current_file_change_map.get_mut(file_name).unwrap();
                change.mode = mode.to_string();
            } else {
                let change = FileChange {
                    added: 0,
                    deleted: 0,
                    file: file_name.to_string(),
                    mode: mode.to_string(),
                };
                self.current_file_change_map
                    .insert(file_name.to_string(), change);
            }
        }
    }

    fn create_file_change(filename: String, caps: Captures) -> FileChange {
        FileChange {
            added: caps["added"].parse::<i32>().unwrap(),
            deleted: caps["deleted"].parse::<i32>().unwrap(),
            file: filename,
            mode: "".to_string(),
        }
    }

    fn create_commit(text: &str, captures: &Captures) -> CocoCommit {
        let commit_id = &captures["commit_id"];
        let without_rev = text
            .split(&format!("[{}] ", commit_id))
            .collect::<Vec<&str>>()[1];

        let author = &AUTHOR.captures(without_rev).unwrap()["author"];
        let without_author = without_rev
            .split(&format!("{} ", author))
            .collect::<Vec<&str>>()[1];

        let date_str = &DATE.captures(without_author).unwrap()["date"];
        let without_date = without_author
            .split(&format!("{} ", date_str))
            .collect::<Vec<&str>>()[1];

        let message = without_date;

        let date = date_str.parse::<i64>().unwrap();
        CocoCommit {
            branch: "".to_string(),
            commit_id: commit_id.to_string(),
            author: author.to_string(),
            committer: "".to_string(),
            date,
            message: message.to_string(),
            changes: vec![],
        }
    }
}

#[cfg(test)]
mod test {
    use crate::infrastructure::git::git_message_parser::GitMessageParser;
    use regex::Regex;

    #[test]
    pub fn should_success_parse_one_line_log() {
        let input = "[828fe39523] Rossen Stoyanchev 1575388800 Consistently use releaseBody in DefaultWebClient
5       3       spring-webflux/core/main/java/org/springframework/web/reactive/function/client/ClientResponse.java
1       1       spring-webflux/core/main/java/org/springframework/web/reactive/function/client/DefaultWebClient.java
9       3       spring-webflux/core/main/java/org/springframework/web/reactive/function/client/WebClient.java
6       11      core/docs/asciidoc/web/webflux-webclient.adoc
";

        let commits = GitMessageParser::parse(input);
        assert_eq!(1, commits.len());
        assert_eq!("828fe39523", commits[0].commit_id);
        assert_eq!("Rossen Stoyanchev", commits[0].author);
        assert_eq!(1575388800, commits[0].date);
        assert_eq!(
            "Consistently use releaseBody in DefaultWebClient",
            commits[0].message
        );
        assert_eq!(4, commits[0].changes.len())
    }

    #[test]
    pub fn should_support_mode_change() {
        let input =
            "[1389e51] Phodal Huang<h@phodal.com> 1606612935 (52d26f5 1389e51,52d26f5) build: init package 20      0       package.json";

        let regex = Regex::new(
            r"(?x)
\[(?P<commit_id>[\d|a-f]{5,12})\]
\s(?P<author>.*?)<(?P<email>.*?)>
\s(?P<date>\d{10})
\s\((?P<parents>([\d|a-f]{5,12}|\s)*),(?P<tree>[\d|a-f]{5,12})\) # parents hash + tree hash
\s.* # commit messages",
        )
        .unwrap();
        match regex.captures(input) {
            None => {
                println!("none");
            }
            Some(caps) => {
                println!("{:?}", caps);
            }
        }
    }

    #[test]
    pub fn should_success_parse_multiple_line_log() {
        let input = "[d00f0124d] Phodal Huang 1575388800 update files
0       0       core/domain/bs/BadSmellApp.go

[1d00f0124b] Phodal Huang 1575388800 update files
1       1       cmd/bs.go
0       0       core/domain/bs/BadSmellApp.go

[d00f04111b] Phodal Huang 1575388800 refactor: move bs to adapter
1       1       cmd/bs.go
5       5       core/{domain => adapter}/bs/BadSmellApp.go

[d00f01214b] Phodal Huang 1575388800 update files
1       1       cmd/bs.go
0       0       core/adapter/bs/BadSmellApp.go
";

        let commits = GitMessageParser::parse(input);
        println!("{:?}", commits);
        assert_eq!(4, commits.len());
    }

    #[test]
    pub fn should_support_multiple_change_mode_change() {
        let input = "[828fe39523] Phodal HUANG 1575388800 fix: fix test
7       0       README.md
13      0       learn_go_suite_test.go
3       3       imp/imp_test.go => learn_go_test.go
 create mode 100644 learn_go_suite_test.go
 rename imp/imp_test.go => learn_go_test.go (70%)
 delete mode 100644 adapter/call/visitor/JavaCallVisitor.go

";

        let commits = GitMessageParser::parse(input);
        assert_eq!(5, commits[0].changes.len());
        let mut changes = commits[0].changes.clone();
        changes.sort_by(|a, b| a.file.to_lowercase().cmp(&b.file.to_lowercase()));

        assert_eq!("delete", changes[0].mode);
        assert_eq!("rename", changes[2].mode);
        assert_eq!("create", changes[3].mode);
    }
}
