use std::process::Command;

pub fn get_commit_message(exec_path: Option<String>) -> String {
    // git log --pretty="format:[%h] %aN<%ae> %at (%p,%t) %s" --date=short --numstat --summary --date=unix --reverse --branches
    let mut command = Command::new("git");

    let git_cmd = command
        .arg("log")
        // more in: https://github.com/git/git/blob/master/Documentation/pretty-formats.txt
        // `%H`: commit hash
        // `%an`: author name
        // `%ae`: author email
        // `%at`: author time
        // `%P`: parent hashes
        // `%T`: tree hash
        // `%s`: subject
        .arg("--pretty=\"format:[%h] %aN<%ae> %at (%p,%t) %s\"")
        .arg("--date=short")
        .arg("--numstat")
        .arg("--reverse")
        .arg("--summary")
        .arg("--date=unix");

    if let Some(path) = exec_path {
        git_cmd.arg("-C").arg(path);
    }

    let output = git_cmd.output().expect("ls command failed to start");

    return format!("{:?}", output.stdout);
}

#[cfg(test)]
mod test {
    use crate::infrastructure::git::git_command::get_commit_message;

    #[test]
    fn should_get_commit_log() {
        let output = get_commit_message(None);
        assert!(output.len() > 1000);
    }
}
