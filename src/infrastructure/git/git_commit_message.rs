use regex::Regex;

lazy_static! {
    static ref CONVENTIONAL: Regex = Regex::new(
        r"(?x)(?P<type>build|chore|ci|docs|feat|fix|perf|refactor|revert|style|test) # type
(?P<scope>(?:\([^()\r\n]*\)|\()?(?P<breaking>!)?)  # scope
:\s?
(?P<subject>.*)? # message
"
    )
    .unwrap();
}

pub struct ConventionalLog {
    pub type_: String,
    pub scope: String,
    pub breaking: bool,
    pub subject: String,
}

pub fn parse_builtin(message: &str) {
    if let Some(capts) = CONVENTIONAL.captures(message) {
        println!("{:?}", capts);
    }
}

#[cfg(test)]
mod test {
    use crate::infrastructure::git::git_commit_message::parse_builtin;

    #[test]
    fn should_parse_conventional_message() {
        let message = "build: init project";
        parse_builtin(message);
    }
}
