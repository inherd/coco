use crate::domain::git::coco_tag::CocoTag;
use regex::Regex;

lazy_static! {
    static ref TAG_INFO: Regex = Regex::new(
        r"(?x)
(?P<commit_id>[\d|a-f]{5,12})
\s(?P<date>\d{10})
\s\s\((?P<tags>.*)\)"
    )
    .unwrap();
}

pub struct GitTagParser {
    tags: Vec<CocoTag>,
}

impl Default for GitTagParser {
    fn default() -> Self {
        GitTagParser { tags: vec![] }
    }
}

impl GitTagParser {
    pub fn parse(str: &str) -> Vec<CocoTag> {
        let split = str.split("\n");
        let mut parser = GitTagParser::default();

        for line in split {
            parser.parse_log_by_line(line)
        }

        parser.tags
    }

    pub fn parse_log_by_line(&mut self, text: &str) {
        if let Some(captures) = TAG_INFO.captures(text) {
            let tags = (&captures["tags"]).split(",");
            let commit_id = &captures["commit_id"];
            let date_str = &captures["date"];
            let date = date_str.parse::<i64>().unwrap();

            let mut share_index = 1;
            for tag in tags {
                if !tag.contains("tag:") {
                    continue;
                }

                let tag = tag.split("tag: ").last().unwrap();
                self.tags.push(CocoTag {
                    name: tag.to_string(),
                    display_name: tag.to_string(),
                    commit_id: commit_id.to_string(),
                    date,
                    share_index,
                });
                share_index = share_index + 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::infrastructure::git::git_tag_parser::GitTagParser;

    #[test]
    pub fn should_parse_commit_id() {
        let input = "92fffa9b 1571521692  (tag: v0.21.0)
1fec6a3c 1570655888
71db1ab2 1541570931";

        let tags = GitTagParser::parse(input);
        assert_eq!(1, tags.len());
        assert_eq!("92fffa9b", &tags[0].commit_id);
        assert_eq!(1571521692, tags[0].date);
        assert_eq!("v0.21.0", tags[0].name);
    }

    #[test]
    pub fn should_not_parse_branch() {
        let input = "817b444 1611642635  (origin/add-license-1)";
        let tags = GitTagParser::parse(input);

        assert_eq!(0, tags.len());
    }

    #[test]
    pub fn should_parse_multiple_tags() {
        let input = "0f152d07 1582212561  (tag: v0.34.0, tag: std/0.34.0)";
        let tags = GitTagParser::parse(input);

        assert_eq!(2, tags.len());
        assert_eq!("std/0.34.0", tags[1].name);
    }
}
