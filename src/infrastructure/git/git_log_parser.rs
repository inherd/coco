use std::collections::HashMap;

use regex::{Captures, Regex};

use crate::domain::git::coco_commit::FileChange;
use crate::domain::git::CocoCommit;

lazy_static! {
    static ref COMMIT_INFO: Regex = Regex::new(
        r"(?x)
\[(?P<commit_id>[\d|a-f]{5,12})\]
\s(?P<author>.*?)<(?P<email>.*?)>
\s(?P<date>\d{10})
\s\((?P<parent_hashes>([\d|a-f]{5,12}|\s)*),(?P<tree_hash>[\d|a-f]{5,12})\) # parents hash + tree hash
\s\#(?P<branch>.*)\#    # branch
\s(?P<message>.*) # commit messages"
    )
    .unwrap();
    static ref CHANGES: Regex =
        Regex::new(r"(?P<added>[\d-]+)[\t\s]+(?P<deleted>[\d-]+)[\t\s]+(?P<filename>.*)").unwrap();
    static ref CHANGEMODEL: Regex =
        Regex::new(r"\s(\w{1,6})\s(mode 100(\d){3})?\s?(.*)(\s\(\d{2}%\))?").unwrap();

    // for rename
    // static ref COMPLEXMOVEREGSTR: Regex = Regex::new(r"(.*)\{(.*)\s=>\s(.*)\}(.*)").unwrap();
    // static ref BASICMOVEREGSTR: Regex = Regex::new(r"(.*)\s=>\s(.*)").unwrap();
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
        if let Some(captures) = COMMIT_INFO.captures(text) {
            self.current_commit = GitMessageParser::create_commit(&captures)
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
        self.current_file_change = vec![];
        for (_filename, change) in &self.current_file_change_map {
            self.current_file_change.push(change.clone());
        }

        self.current_commit.changes = self.current_file_change.clone();

        self.current_commit.changed_file_count = self.current_commit.changes.len() as i32;
        self.current_commit.total_added = 0;
        self.current_commit.total_deleted = 0;

        for change in &self.current_commit.changes {
            self.current_commit.total_added = self.current_commit.total_added + change.added;
            self.current_commit.total_deleted = self.current_commit.total_deleted + change.deleted;
        }

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
        let mut added = 0;
        let mut deleted = 0;

        if let Ok(value) = caps["added"].parse::<i32>() {
            added = value
        }

        if let Ok(value) = caps["deleted"].parse::<i32>() {
            deleted = value
        }

        FileChange {
            added,
            deleted,
            file: filename,
            mode: "".to_string(),
        }
    }

    fn create_commit(captures: &Captures) -> CocoCommit {
        let commit_id = &captures["commit_id"];
        let author = &captures["author"];
        let date_str = &captures["date"];
        let message = &captures["message"];
        let email = &captures["email"];
        let branch = &captures["branch"];

        let mut parent_hashes = vec![];
        if let Some(_) = captures.name("parent_hashes") {
            let hashes = &captures["parent_hashes"];
            if hashes != "" {
                parent_hashes = hashes.split(" ").map(|str| str.to_string()).collect()
            }
        }

        let tree_hash = captures["tree_hash"].to_string();

        let date = date_str.parse::<i64>().unwrap();
        CocoCommit {
            branch: branch.to_string(),
            commit_id: commit_id.to_string(),
            author: author.to_string(),
            email: email.to_string(),
            committer: "".to_string(),
            date,
            message: message.to_string(),
            changes: vec![],
            parent_hashes,
            tree_hash,
            total_added: 0,
            total_deleted: 0,
            changed_file_count: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::infrastructure::git::git_log_parser::GitMessageParser;

    #[test]
    pub fn should_success_parse_one_line_log() {
        let input = "[828fe39523] Phodal Huang<h@phodal.com> 1606612935 (52d26f5 1389e51,52d26f5) #origin/main# Consistently use releaseBody in DefaultWebClient
5       3       spring-webflux/core/main/java/org/springframework/web/reactive/function/client/ClientResponse.java
1       1       spring-webflux/core/main/java/org/springframework/web/reactive/function/client/DefaultWebClient.java
9       3       spring-webflux/core/main/java/org/springframework/web/reactive/function/client/WebClient.java
6       11      core/docs/asciidoc/web/webflux-webclient.adoc
";

        let commits = GitMessageParser::parse(input);
        assert_eq!(1, commits.len());
        assert_eq!("828fe39523", commits[0].commit_id);
        assert_eq!("Phodal Huang", commits[0].author);
        assert_eq!(1606612935, commits[0].date);
        assert_eq!(
            "Consistently use releaseBody in DefaultWebClient",
            commits[0].message
        );
        assert_eq!(4, commits[0].changes.len())
    }

    #[test]
    pub fn should_success_parse_multiple_line_log() {
        let input = "[d00f0124d] Phodal Huang<h@phodal.com> 1606612935 (52d26f5 1389e51,52d26f5) #origin/main# update files
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
        assert_eq!(4, commits.len());
    }

    #[test]
    pub fn should_support_multiple_change_mode_change() {
        let input = "[828fe39523] Phodal Huang<h@phodal.com> 1606612935 (52d26f5 1389e51,52d26f5) #origin/main# fix: fix test
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

    #[test]
    pub fn should_support_parent_hashes() {
        let input =
            "[1389e51] Phodal Huang<h@phodal.com> 1606612935 (52d26f5 1389e51,52d26f5) #origin/main# build: init package
20      0       package.json
";

        let commits = GitMessageParser::parse(input);

        assert_eq!(2, commits[0].parent_hashes.len());
        assert_eq!("52d26f5", commits[0].parent_hashes[0]);
        assert_eq!("1389e51", commits[0].parent_hashes[1]);
        assert_eq!("52d26f5", commits[0].tree_hash);
    }

    #[test]
    pub fn should_handle_empty_parents() {
        let input =
            "[1389e51] Phodal Huang<h@phodal.com> 1606612935 (,52d26f5) #origin/main# build: init package
20      0       package.json
";

        let commits = GitMessageParser::parse(input);

        assert_eq!(0, commits[0].parent_hashes.len());
        assert_eq!(1, commits[0].changed_file_count);
    }

    #[test]
    pub fn should_find_file_change() {
        let input = "[828fe39523] Phodal Huang<h@phodal.com> 1606612935 (52d26f5 1389e51,52d26f5) #origin/main# fix: fix test
7       0       README.md
13      0       learn_go_suite_test.go
3       3       imp/imp_test.go => learn_go_test.go
 create mode 100644 learn_go_suite_test.go
 rename imp/imp_test.go => learn_go_test.go (70%)
 delete mode 100644 adapter/call/visitor/JavaCallVisitor.go

";

        let commits = GitMessageParser::parse(input);
        assert_eq!(23, commits[0].total_added);
        assert_eq!(3, commits[0].total_deleted);
        assert_eq!("h@phodal.com", commits[0].email);
        assert_eq!(5, commits[0].changed_file_count);
    }

    #[test]
    pub fn should_support_bin_file_added() {
        let input = "[3eff25c] yvettemuki<373628977@qq.com> 1612073308 (,f3e348c) ## initialize
-       -       build/CMakeFiles/3.19.0/CMakeDetermineCompilerABI_C.bin
-       -       build/CMakeFiles/3.19.0/CMakeDetermineCompilerABI_CXX.bin
 create mode 100755 build/CMakeFiles/3.19.0/CMakeDetermineCompilerABI_C.bin
 create mode 100755 build/CMakeFiles/3.19.0/CMakeDetermineCompilerABI_CXX.bin
";

        let commits = GitMessageParser::parse(input);
        assert_eq!(2, commits[0].changes.len());
        assert_eq!(0, commits[0].total_added);
        assert_eq!(0, commits[0].total_deleted);
    }
}
