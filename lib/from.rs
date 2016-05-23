extern crate rustc_serialize;

use error::CSDError;
use exec::*;

use std::fs::File;
use std::io::prelude::*;
use std::fmt::Debug;

use rustc_serialize::{Decodable, json};

// Generic trait to read file to serializable struct
pub trait FromFile<T> {
    fn from_file(path: &str) -> Result<T, CSDError>;
}

impl <T: Decodable + Debug> FromFile<T> for T {
    fn from_file(path: &str) -> Result<T, CSDError> {
        let mut file = try!(File::open(path));
        let mut buffer = String::new();
        try!(file.read_to_string(&mut buffer));
        Ok(try!(json::decode(&buffer)))
    }
}

pub trait FromCeph<T> {
    fn from_ceph(cmd: &str) -> Result<T, CSDError>;
}

impl <T: Decodable + Debug> FromCeph<T> for T {
    fn from_ceph(cmd: &str) -> Result<T, CSDError> {
        Ok(try!(json::decode(&try!(call_ceph(cmd)))))
    }
}

