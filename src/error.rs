use std::{io, fmt};
use zip::result::ZipError;
use reqwest;

// Todo: optimize this mod

/// Base Error
pub struct CocoError {
    msg: String,
}

impl fmt::Display for CocoError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "CocoError: {}", self.msg)
    }
}

impl From<io::Error> for CocoError {
    fn from(err: io::Error) -> Self {
        Self { msg: format!("cause by: {}", err) }
    }
}

impl From<ZipError> for CocoError {
    fn from(err: ZipError) -> Self {
        Self { msg: format!("cause by: {}", err) }
    }
}

impl From<reqwest::Error> for CocoError {
    fn from(err: reqwest::Error) -> Self {
        Self { msg: format!("cause by: {}", err) }
    }
}

impl From<CocoError> for io::Error {
    fn from(err: CocoError) -> io::Error {
        io::Error::new(io::ErrorKind::Other, &*(err.to_string()))
    }
}
