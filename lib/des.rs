extern crate serde;

use std::fmt;
use self::serde::{de, Deserializer};


// Deserializes `str` types to bool
struct DeserializetoBool;

impl de::Visitor for DeserializetoBool {
    type Value = bool;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an boolean value")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where E: de::Error
    {
        Ok(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where E: de::Error
    {
        Ok(v.parse::<bool>().unwrap_or(false))
    }
}

pub fn to_bool<D>(des: D) -> Result<bool, D::Error>
    where D: Deserializer
{
    des.deserialize(DeserializetoBool)
}


// Deserializes `str` types to f64
struct DeserializetoF64;

impl de::Visitor for DeserializetoF64 {
    type Value = f64;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an floating point integer")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where E: de::Error
    {
        Ok(v.parse::<f64>().unwrap_or(0.0_f64))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where E: de::Error
    {
        Ok(v)
    }
}

pub fn to_f64<D>(des: D) -> Result<f64, D::Error>
    where D: Deserializer
{
    des.deserialize(DeserializetoF64)
}
