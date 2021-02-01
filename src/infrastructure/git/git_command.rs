use std::process::Command;

pub fn get_commit_message() -> String {
    // git log --pretty="format:%H%n%aN%n%at%n%P%n%T%n--------%n%s" --date=short --numstat --summary --date=unix --reverse
    let command = Command::new("git")
        .arg("log")
        // -------- is the placeholder for parser object
        .arg("--pretty=\"format:%H%n%aN%n%at%n%P%n%T%n--------%n%s\"")
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
