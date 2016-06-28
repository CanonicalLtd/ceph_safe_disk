extern crate serde_json;

use std::fmt;
use std::io;
use std::string;

use serde_json::error;

#[derive(Debug)]
pub enum CSDError {
    Io(io::Error),
    JsonDecode(error::Error),
    Utf8Error(string::FromUtf8Error),
    CephExecError(String),
}

impl fmt::Display for CSDError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CSDError::Io(ref err) => write!(f , "I/O error, {}", err),
            CSDError::JsonDecode(ref err) => write!(f, "JSON decoding error, {}", err),
            CSDError::Utf8Error(ref err) => write!(f, "UTF-8 conversion error, {}", err),
            CSDError::CephExecError(ref err) => write!(f, "Error executing `ceph`, {}", err),
        }
    }
}

impl From<io::Error> for CSDError {
    fn from(err: io::Error) -> CSDError {
        CSDError::Io(err)
    }
}

impl From<error::Error> for CSDError {
    fn from(err: error::Error) -> CSDError {
        CSDError::JsonDecode(err)
    }
}

impl From<string::FromUtf8Error> for CSDError {
    fn from(err: string::FromUtf8Error) -> CSDError {
        CSDError::Utf8Error(err)
    }
}
