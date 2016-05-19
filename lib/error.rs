extern crate rustc_serialize;

use std::fmt;
use std::io;

use rustc_serialize::json;

#[derive(Debug)]
pub enum CSDError {
    Io(io::Error),
    JsonDecode(json::DecoderError),
}

impl fmt::Display for CSDError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CSDError::Io(ref err) => write!(f , "I/O error, {}", err),
            CSDError::JsonDecode(ref err) => write!(f, "JSON decoding error, {}", err),
        }
    }
}

impl From<io::Error> for CSDError {
    fn from(err: io::Error) -> CSDError {
        CSDError::Io(err)
    }
}

impl From<json::DecoderError> for CSDError {
    fn from(err: json::DecoderError) -> CSDError {
        CSDError::JsonDecode(err)
    }
}
