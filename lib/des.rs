extern crate serde;

use serde::{de, Deserializer};


// Deserializes `str` types to bool
struct DeserializetoBool;

impl de::Visitor for DeserializetoBool {
    type Value = bool;

    fn visit_bool<E>(&mut self, v: bool) -> Result<Self::Value, E>
        where E: de::Error
    {
        Ok(v)
    }

    fn visit_str<E>(&mut self, v: &str) -> Result<Self::Value, E>
        where E: de::Error
    {
        Ok(v.parse::<bool>().unwrap_or(false))
    }
}

pub fn to_bool<D>(des: &mut D) -> Result<bool, D::Error>
    where D: Deserializer
{
    des.deserialize(DeserializetoBool)
}

// Deserializes `str` types to f64
struct DeserializetoF64;

impl de::Visitor for DeserializetoF64 {
    type Value = f64;

    fn visit_str<E>(&mut self, v: &str) -> Result<Self::Value, E>
        where E: de::Error
    {
        Ok(v.parse::<f64>().unwrap_or(0.0_f64))
    }

    fn visit_f64<E>(&mut self, v: f64) -> Result<Self::Value, E>
        where E: de::Error
    {
        Ok(v)
    }
}

pub fn to_f64<D>(des: &mut D) -> Result<f64, D::Error>
    where D: Deserializer
{
    des.deserialize(DeserializetoF64)
}
