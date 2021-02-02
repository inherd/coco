use std::process::Command;
use std::str;

pub fn get_commit_message(exec_path: Option<String>) -> String {
    // git log --pretty="format:[%h] %aN<%ae> %at (%p,%t) #%S# %s" --date=short --numstat --summary --date=unix --reverse --branches --remotes
    // more docs see in: https://git-scm.com/docs/git-log#_pretty_formats
    let mut command = Command::new("git");

    if let Some(path) = exec_path {
        command.arg("-C").arg(path);
    }

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
        .arg("--pretty=format:[%h] %aN<%ae> %at (%p,%t) #%S# %s")
        .arg("--date=short")
        .arg("--numstat")
        .arg("--reverse")
        .arg("--summary")
        .arg("--date=unix")
        .arg("--branches")
        .arg("--remotes");

    let output = git_cmd.output().expect("ls command failed to start");
    return str::from_utf8(&*output.stdout).unwrap().to_string();
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
