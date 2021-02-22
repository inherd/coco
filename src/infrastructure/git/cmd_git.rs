use std::process::Command;

/// Return project's tags in String
///  from: git log --tags --simplify-by-decoration --pretty="format:%t %at %d"
/// # Arguments
/// * `exec_path` - Option<String> for path
pub fn tags(exec_path: Option<String>) -> String {
    let mut command = Command::new("git");

    if let Some(path) = exec_path {
        command.arg("-C").arg(path);
    }

    let git_cmd = command
        .arg("log")
        .arg("--tags")
        .arg("--simplify-by-decoration")
        // `%at`: author time in Unix
        // `%t`: tree hash in short
        // ref names, like the --decorate option of
        .arg("--pretty=format:%t %at %d");

    let output = git_cmd.output().expect("ls command failed to start");
    return String::from_utf8_lossy(&*output.stdout).to_string();
}

/// Return project's commits in String
/// git log --pretty="format:[%h] %aN<%ae> %at (%p,%t) #%gn# %s" --date=short --numstat --summary --date=unix --reverse --branches --remotes
/// more docs see in: https://github.com/git/git/blob/master/Documentation/pretty-formats.txt
pub fn commit_message(exec_path: Option<String>) -> String {
    let mut command = Command::new("git");

    if let Some(path) = exec_path {
        command.arg("-C").arg(path);
    }

    let git_cmd = command
        .arg("log")
        // `%H`: commit hash
        // `%an`: author name
        // `%ae`: author email
        // `%at`: author time
        // `%P`: parent hashes
        // `%t`: tree hash in short
        // `%s`: subject
        // todo: resolve ref names  issues
        .arg("--pretty=format:[%h] %aN<%ae> %at (%p,%t) #%S# %s")
        .arg("--date=short")
        .arg("--numstat")
        .arg("--reverse")
        .arg("--summary")
        .arg("--date=unix")
        .arg("--branches")
        .arg("--remotes");

    let output = git_cmd.output().expect("ls command failed to start");
    return String::from_utf8_lossy(&*output.stdout).to_string();
}

#[cfg(test)]
mod test {
    use crate::infrastructure::git::cmd_git;

    #[test]
    fn should_get_commit_log() {
        let output = cmd_git::commit_message(None);
        assert!(output.len() > 1000);
    }

    #[ignore]
    #[test]
    fn should_get_git_tag() {
        let output = cmd_git::tags(None);
        assert!(output.contains("0.1.3"));
    }
}
