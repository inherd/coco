use std::process::Command;

pub fn get_commit_message() -> String {
    // git log --pretty="format:[%H] %aN<%ae> %at (%P,%T) %s" --date=short --numstat --summary --date=unix --reverse
    let command = Command::new("git")
        .arg("log")
        // `%H`: commit hash
        // `%an`: author name
        // `%ae`: author email
        // `%at`: author time
        // `%P`: parent hashes
        // `%T`: tree hash
        // `%s`: subject
        .arg("--pretty=\"format:[%H] %aN<%ae> %at (%P,%T) %s\"")
        .arg("--date=short")
        .arg("--numstat")
        .arg("--reverse")
        .arg("--summary")
        .arg("--date=unix")
        .output()
        .expect("ls command failed to start");

    return format!("{:?}", command.stdout);
}

#[cfg(test)]
mod test {
    use crate::infrastructure::git::git_command::get_commit_message;

    #[test]
    fn should_get_commit_log() {
        let output = get_commit_message();
        assert!(output.len() > 1000);
    }
}
