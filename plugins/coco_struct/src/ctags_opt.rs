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
use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;
use structopt::{clap, StructOpt};
use structopt_toml::StructOptToml;

#[derive(Debug, Deserialize, Serialize, StructOpt, StructOptToml)]
#[serde(default)]
#[structopt(name = "ptags")]
#[structopt(long_version = option_env!("LONG_VERSION").unwrap_or(env!("CARGO_PKG_VERSION")))]
#[structopt(setting = clap::AppSettings::AllowLeadingHyphen)]
#[structopt(setting = clap::AppSettings::ColoredHelp)]
pub struct Opt {
    /// Number of threads
    #[structopt(short = "t", long = "thread", default_value = "8")]
    pub thread: usize,

    /// Output filename ( filename '-' means output to stdout )
    #[structopt(short = "f", long = "file", default_value = "tags", parse(from_os_str))]
    pub output: PathBuf,

    /// Search directory
    #[structopt(name = "DIR", default_value = ".", parse(from_os_str))]
    pub dir: PathBuf,

    /// Show statistics
    #[structopt(short = "s", long = "stat")]
    pub stat: bool,

    /// Filename of input file list
    #[structopt(short = "L", long = "list")]
    pub list: Option<String>,

    /// Path to ctags binary
    #[structopt(long = "bin-ctags", default_value = "ctags", parse(from_os_str))]
    pub bin_ctags: PathBuf,

    /// Path to git binary
    #[structopt(long = "bin-git", default_value = "git", parse(from_os_str))]
    pub bin_git: PathBuf,

    /// Options passed to ctags
    #[structopt(short = "c", long = "opt-ctags", number_of_values = 1)]
    pub opt_ctags: Vec<String>,

    /// Options passed to git
    #[structopt(short = "g", long = "opt-git", number_of_values = 1)]
    pub opt_git: Vec<String>,

    /// Options passed to git-lfs
    #[structopt(long = "opt-git-lfs", number_of_values = 1)]
    pub opt_git_lfs: Vec<String>,

    /// Verbose mode
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,

    /// Exclude git-lfs tracked files
    #[structopt(long = "exclude-lfs")]
    pub exclude_lfs: bool,

    /// Include untracked files
    #[structopt(long = "include-untracked")]
    pub include_untracked: bool,

    /// Include ignored files
    #[structopt(long = "include-ignored")]
    pub include_ignored: bool,

    /// Include submodule files
    #[structopt(long = "include-submodule")]
    pub include_submodule: bool,

    /// Validate UTF8 sequence of tag file
    #[structopt(long = "validate-utf8")]
    pub validate_utf8: bool,

    /// Disable tags sort
    #[structopt(long = "unsorted")]
    pub unsorted: bool,

    /// Disable tags sort
    #[structopt(long = "fields")]
    pub fields: Option<String>,

    /// Languages
    #[structopt(long = "languages")]
    pub languages: Option<String>,

    /// Glob pattern of exclude file ( ex. --exclude '*.rs' )
    #[structopt(short = "e", long = "exclude", number_of_values = 1)]
    pub exclude: Vec<String>,

    /// Generate shell completion file
    #[structopt(
    long = "completion",
    possible_values = &["bash", "fish", "zsh", "powershell"]
    )]
    pub completion: Option<String>,

    /// Generate configuration sample file
    #[structopt(long = "config")]
    pub config: bool,
}
