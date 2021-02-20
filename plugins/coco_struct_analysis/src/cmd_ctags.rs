/// MIT License
//
// Copyright (c) 2018 dalance <dalance@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
// https://github.com/dalance/ptags
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Write};
#[cfg(target_os = "linux")]
use std::os::unix::io::AsRawFd;
use std::path::PathBuf;
use std::process::{ChildStdin, Command, Output, Stdio};
use std::str;
use std::sync::mpsc;
use std::thread;

use failure::{bail, Error, Fail, ResultExt};
#[cfg(target_os = "linux")]
use nix::fcntl::{fcntl, FcntlArg};
use tempfile::NamedTempFile;

use crate::option_ctags::Opt;

// ---------------------------------------------------------------------------------------------------------------------
// Error
// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Fail)]
enum CtagsError {
    #[fail(display = "failed to execute ctags command ({})\n{}", cmd, err)]
    ExecFailed { cmd: String, err: String },

    #[fail(display = "failed to call ctags command ({})", cmd)]
    CallFailed { cmd: String },

    #[fail(display = "failed to convert to UTF-8 ({:?})", s)]
    ConvFailed { s: Vec<u8> },
}

// ---------------------------------------------------------------------------------------------------------------------
// CmdCtags
// ---------------------------------------------------------------------------------------------------------------------

pub struct CmdCtags;

impl CmdCtags {
    pub fn call(opt: &Opt, files: &[String]) -> Result<Vec<Output>, Error> {
        let mut args = Vec::new();
        args.push(String::from("-L -"));
        args.push(String::from("-f -"));
        if opt.unsorted {
            args.push(String::from("--sort=no"));
        }
        for e in &opt.exclude {
            args.push(String::from(format!("--exclude={}", e)));
        }
        args.append(&mut opt.opt_ctags.clone());

        let cmd = CmdCtags::get_cmd(&opt, &args);

        let (tx, rx) = mpsc::channel::<Result<Output, Error>>();

        for i in 0..opt.thread {
            let tx = tx.clone();
            let file = files[i].clone();
            let dir = opt.dir.clone();
            let bin_ctags = opt.bin_ctags.clone();
            let args = args.clone();
            let cmd = cmd.clone();

            if opt.verbose {
                eprintln!("Call : {}", cmd);
            }

            thread::spawn(move || {
                let child = Command::new(bin_ctags.clone())
                    .args(args)
                    .current_dir(dir)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    //.stderr(Stdio::piped()) // Stdio::piped is x2 slow to wait_with_output() completion
                    .stderr(Stdio::null())
                    .spawn();
                match child {
                    Ok(mut x) => {
                        {
                            let stdin = x.stdin.as_mut().unwrap();
                            let pipe_size = std::cmp::min(file.len() as i32, 1048576);
                            let _ = CmdCtags::set_pipe_size(&stdin, pipe_size)
                                .or_else(|x| tx.send(Err(x.into())));
                            let _ = stdin.write_all(file.as_bytes());
                        }
                        match x.wait_with_output() {
                            Ok(x) => {
                                let _ = tx.send(Ok(x));
                            }
                            Err(x) => {
                                let _ = tx.send(Err(x.into()));
                            }
                        }
                    }
                    Err(_) => {
                        let _ = tx.send(Err(CtagsError::CallFailed { cmd }.into()));
                    }
                }
            });
        }

        let mut children = Vec::new();
        for _ in 0..opt.thread {
            children.push(rx.recv());
        }

        let mut outputs = Vec::new();
        for child in children {
            let output = child??;

            if !output.status.success() {
                bail!(CtagsError::ExecFailed {
                    cmd: cmd,
                    err: String::from(str::from_utf8(&output.stderr).context(
                        CtagsError::ConvFailed {
                            s: output.stderr.to_vec(),
                        }
                    )?)
                });
            }

            outputs.push(output);
        }

        Ok(outputs)
    }

    pub fn get_tags_header(opt: &Opt) -> Result<String, Error> {
        let tmp_empty = NamedTempFile::new()?;
        let tmp_tags = NamedTempFile::new()?;
        let tmp_tags_path: PathBuf = tmp_tags.path().into();
        // In windiws environment, write access by ctags to the opened tmp_tags fails.
        // So the tmp_tags must be closed and deleted.
        tmp_tags.close()?;

        let _ = Command::new(&opt.bin_ctags)
            .arg(format!("-L {}", tmp_empty.path().to_string_lossy()))
            .arg(format!("-f {}", tmp_tags_path.to_string_lossy()))
            .args(&opt.opt_ctags)
            .current_dir(&opt.dir)
            .status();
        let mut f = BufReader::new(File::open(&tmp_tags_path)?);
        let mut s = String::new();
        f.read_to_string(&mut s)?;

        fs::remove_file(&tmp_tags_path)?;

        Ok(s)
    }

    fn get_cmd(opt: &Opt, args: &[String]) -> String {
        let mut cmd = format!(
            "cd {}; {}",
            opt.dir.to_string_lossy(),
            opt.bin_ctags.to_string_lossy()
        );
        for arg in args {
            cmd = format!("{} {}", cmd, arg);
        }
        cmd
    }

    #[allow(dead_code)]
    fn is_exuberant_ctags(opt: &Opt) -> Result<bool, Error> {
        let output = Command::new(&opt.bin_ctags)
            .arg("--version")
            .current_dir(&opt.dir)
            .output()?;
        Ok(str::from_utf8(&output.stdout)?.starts_with("Exuberant Ctags"))
    }

    #[cfg(target_os = "linux")]
    fn set_pipe_size(stdin: &ChildStdin, len: i32) -> Result<(), Error> {
        fcntl(stdin.as_raw_fd(), FcntlArg::F_SETPIPE_SZ(len))?;
        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    fn set_pipe_size(_stdin: &ChildStdin, _len: i32) -> Result<(), Error> {
        Ok(())
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// Test
// ---------------------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::str;

    use structopt::StructOpt;

    use crate::option_ctags::Opt;

    use super::CmdCtags;

    #[test]
    fn test_call() {
        let args = vec!["ptags", "-t", "1"];
        let opt = Opt::from_iter(args.iter());
        let mut files = vec![];
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();
        let code_dir = root_dir
            .clone()
            .join("_fixtures")
            .join("ctags")
            .join("main.go");

        files.push(format!("{}", code_dir.display()));
        let outputs = CmdCtags::call(&opt, &files).unwrap();
        let mut iter = str::from_utf8(&outputs[0].stdout).unwrap().lines();

        println!("{:?}", outputs);
        assert!(iter.next().unwrap_or("").contains("age"));
    }

    // #[test]
    // fn test_call_with_opt() {
    //     let args = vec!["ptags", "-t", "1", "--opt-ctags=-u"];
    //     let opt = Opt::from_iter(args.iter());
    //     let files = git_files(&opt).unwrap();
    //     let outputs = CmdCtags::call(&opt, &files).unwrap();
    //     let mut iter = str::from_utf8(&outputs[0].stdout).unwrap().lines();
    //     assert_eq!(
    //         iter.next().unwrap_or(""),
    //         "VERSION\tMakefile\t/^VERSION = $(patsubst \"%\",%, $(word 3, $(shell grep version Cargo.toml)))$/;\"\tm"
    //     );
    // }
    //
    // #[test]
    // fn test_call_exclude() {
    //     let args = vec![
    //         "ptags",
    //         "-t",
    //         "1",
    //         "--exclude=Make*",
    //         "--exclude=README.md",
    //         "-v",
    //     ];
    //     let opt = Opt::from_iter(args.iter());
    //     let files = git_files(&opt).unwrap();
    //     let outputs = CmdCtags::call(&opt, &files).unwrap();
    //     let mut iter = str::from_utf8(&outputs[0].stdout).unwrap().lines();
    //
    //     // Exuberant Ctags doesn't support Rust ( *.rs ).
    //     // So the result becomes empty when 'Makefile' is excluded.
    //     if CmdCtags::is_exuberant_ctags(&opt).unwrap() {
    //         assert_eq!(iter.next().unwrap_or(""), "");
    //     } else {
    //         assert_eq!(
    //             iter.next().unwrap_or(""),
    //             "CallFailed\tsrc/cmd_ctags.rs\t/^    CallFailed { cmd: String },$/;\"\te\tenum:CtagsError"
    //         );
    //     }
    // }
    //
    // #[test]
    // fn test_command_fail() {
    //     let args = vec!["ptags", "--bin-ctags", "aaa"];
    //     let opt = Opt::from_iter(args.iter());
    //     let files = git_files(&opt).unwrap();
    //     let outputs = CmdCtags::call(&opt, &files);
    //     assert_eq!(
    //         &format!("{:?}", outputs),
    //         "Err(CallFailed { cmd: \"cd .; aaa -L - -f -\" })"
    //     );
    // }
    //
    // #[test]
    // fn test_ctags_fail() {
    //     let args = vec!["ptags", "--opt-ctags=--u"];
    //     let opt = Opt::from_iter(args.iter());
    //     let files = git_files(&opt).unwrap();
    //     let outputs = CmdCtags::call(&opt, &files);
    //     assert_eq!(
    //         &format!("{:?}", outputs)[0..74],
    //         "Err(ErrorMessage { msg: ExecFailed { cmd: \"cd .; ctags -L - -f - --u\", err"
    //     );
    // }
    //
    // #[test]
    // fn test_get_tags_header() {
    //     let args = vec!["ptags"];
    //     let opt = Opt::from_iter(args.iter());
    //     let output = CmdCtags::get_tags_header(&opt).unwrap();
    //     let output = output.lines().next();
    //     assert_eq!(&output.unwrap_or("")[0..18], "!_TAG_FILE_FORMAT\t");
    // }
}
