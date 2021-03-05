// MIT License
//
// Copyright (c) 2020 Korny Sietsma
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
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::infrastructure::cloc;
use content_inspector::{inspect, ContentType};
use failure::Error;
use git_scanner::flare::FlareTreeNode;
use git_scanner::git::GitCalculator;
use git_scanner::git_logger::GitLogConfig;
use git_scanner::{file_walker, IndicatorCalculator};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokei::{Config, LanguageType};

#[derive(Debug)]
pub struct LocCalculator {}

const MAX_PEEK_SIZE: usize = 1024;

fn file_content_type(filename: &Path) -> Result<ContentType, Error> {
    let file = File::open(&filename)?;
    let mut buffer: Vec<u8> = vec![];

    file.take(MAX_PEEK_SIZE as u64).read_to_end(&mut buffer)?;
    Ok(inspect(&buffer))
}

fn file_size(filename: &Path) -> Result<u64, Error> {
    Ok(filename.metadata()?.len())
}

fn safe_extension(filename: &Path) -> String {
    match filename.extension() {
        Some(ext) => ext.to_string_lossy().to_string(),
        None => "no_extension".to_owned(),
    }
}

/// a struct representing tokei language data - based on tokei::Stats and tokei::Languages::name
#[derive(Debug, PartialEq, Serialize)]
struct LanguageLocData {
    /// Canonical language name
    pub language: String,
    /// binary files only have bytes not lines!
    pub binary: bool,
    /// Number of blank lines within the file.
    pub blanks: usize,
    /// Number of lines of code within the file.
    pub code: usize,
    /// Number of comments within the file. (_includes both multi line, and
    /// single line comments_)
    pub comments: usize,
    /// Total number of lines within the file.
    pub lines: usize,
    /// File size in bytes
    pub bytes: u64,
}

impl Default for LanguageLocData {
    fn default() -> Self {
        LanguageLocData {
            language: "".to_string(),
            binary: true,
            blanks: 0,
            code: 0,
            comments: 0,
            lines: 0,
            bytes: 0,
        }
    }
}
impl LanguageLocData {
    fn from_binary(language_name: String, filename: &Path) -> Result<Self, Error> {
        Ok(LanguageLocData {
            language: language_name,
            binary: true,
            blanks: 0,
            code: 0,
            comments: 0,
            lines: 0,
            bytes: file_size(filename)?,
        })
    }
}

/// A struct representing the statistics of a file.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Stats {
    /// Number of blank lines within the file.
    pub blanks: usize,
    /// Number of lines of code within the file.
    pub code: usize,
    /// Number of comments within the file. (_includes both multi line, and
    /// single line comments_)
    pub comments: usize,
    /// Total number of lines within the file.
    pub lines: usize,
    /// File name.
    pub name: PathBuf,
}

#[allow(unused_assignments)]
fn parse_file(filename: &Path) -> Result<LanguageLocData, Error> {
    let config = Config::default();
    let mut language_name = None;
    match LanguageType::from_path(filename, &config) {
        Some(language) => language,
        None => {
            language_name = Some(safe_extension(filename));
            if file_content_type(filename)? == ContentType::BINARY {
                return LanguageLocData::from_binary(language_name.unwrap(), filename);
            }
            LanguageType::Text
        }
    };

    // todo: refactor;
    let mut data = LanguageLocData::default();
    for (lang_type, language) in cloc::by_dir(&filename) {
        data = LanguageLocData {
            language: lang_type.to_string(),
            binary: false,
            blanks: language.blanks,
            code: language.code,
            comments: language.comments,
            lines: language.code,
            bytes: file_size(filename)?,
        };
    }

    return Ok(data);
}

impl IndicatorCalculator for LocCalculator {
    fn name(&self) -> String {
        "loc".to_string()
    }

    fn calculate(&mut self, path: &Path) -> Result<Option<serde_json::Value>, Error> {
        if path.is_file() {
            let stats = parse_file(path)?;
            Ok(Some(serde_json::value::to_value(stats).expect(
                "Serializable object couldn't be serialized to JSON",
            ))) // TODO: maybe explicit error? Though this should be fatal
        } else {
            Ok(None)
        }
    }

    fn metadata(&self) -> Result<Option<Value>, Error> {
        Ok(None)
    }
}

pub fn by_path(root: PathBuf, git_years: f64) -> FlareTreeNode {
    let mut tics: Vec<Box<dyn IndicatorCalculator>> = vec![];
    let calculator = Box::new(GitCalculator::new(
        GitLogConfig::default()
            .include_merges(true)
            .since_years(git_years),
        true,
    ));

    let loc_calculator = Box::new(LocCalculator {});

    tics.push(calculator);
    tics.push(loc_calculator);

    let mut tree = file_walker::walk_directory(&root, &mut tics).unwrap();

    for tic in tics {
        if let Some(metadata) = tic.metadata().unwrap() {
            tree.add_data(tic.name() + "_meta", metadata);
        }
    }

    return tree;
}
