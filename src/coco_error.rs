use reqwest;
use std::{fmt, io};
use zip::result::ZipError;

// Todo: optimize this mod

/// Base Error
pub struct CocoError {
    msg: String,
}

impl CocoError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: String::from(msg),
        }
    }
}

impl fmt::Display for CocoError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "CocoError: {}", self.msg)
    }
}

impl From<io::Error> for CocoError {
    fn from(err: io::Error) -> Self {
        Self {
            msg: format!("cause by: {}", err),
        }
    }
}

impl From<ZipError> for CocoError {
    fn from(_err: ZipError) -> Self {
        Self {
            msg: String::from("cause by: unzip error"),
        }
    }
}

impl From<reqwest::Error> for CocoError {
    fn from(err: reqwest::Error) -> Self {
        Self {
            msg: format!("cause by: {}", err),
        }
    }
}
