use crate::domain::git::coco_commit_message::ConventionalMessage;
use regex::Regex;

lazy_static! {
    static ref CONVENTIONAL: Regex = Regex::new(
        r"(?x)(?P<type>build|chore|ci|docs|feat|fix|perf|refactor|revert|style|test) # type
(?:\((?P<scope>[^()\r\n]*)\)|\()?(?P<breaking>!)? # scope
:\s?
(?P<subject>.*)? # message
"
    )
    .unwrap();
}

pub fn parse_builtin(message: &str) -> Option<ConventionalMessage> {
    let mut commit_message = ConventionalMessage::default();
    if let Some(caps) = CONVENTIONAL.captures(message) {
        commit_message.type_ = (&caps["type"]).to_string();
        commit_message.subject = (&caps["subject"]).to_string();
        if let Some(_breaking) = caps.name("breaking") {
            commit_message.breaking = true;
        }
        if let Some(_scope) = caps.name("scope") {
            commit_message.scope = (&caps["scope"]).to_string();
        }

        return Some(commit_message);
    }

    return None;
}

#[cfg(test)]
mod test {
    use crate::infrastructure::git::git_commit_message::parse_builtin;

    #[test]
    fn should_parse_normal_conventional_message() {
        let msg = parse_builtin("build(visual): init project").unwrap();
        assert_eq!("build", msg.type_);
        assert_eq!("init project", msg.subject);
        assert_eq!("visual", msg.scope);
        assert_eq!(false, msg.breaking);
    }

    #[test]
    fn should_parse_breaking_conventional_message() {
        let msg = parse_builtin("build(visual)!: init project").unwrap();
        assert_eq!("build", msg.type_);
        assert_eq!("init project", msg.subject);
        assert_eq!("visual", msg.scope);
        assert_eq!(true, msg.breaking);
    }
}
