use std::{collections::BTreeMap, fmt::Display};

use serde_json::Value;

pub enum Json {
    Array(Vec<Json>),
    Boolean(bool),
    Object(BTreeMap<String, Json>),
    Number(JsonNumber),
    Null,
    String(String),
}

#[derive(PartialEq, Debug)]
pub enum JsonNumber {
    UInt(usize),
    Int(isize),
    Float(f64),
}

#[derive(Debug)]
pub struct AsFloatError;
impl Display for AsFloatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: {} ", self.to_string())
    }
}
#[derive(Debug)]
pub struct AsIntError;
impl Display for AsIntError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: {} ", self.to_string())
    }
}
#[derive(Debug)]
pub struct AsUIntError;
impl Display for AsUIntError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: {} ", self.to_string())
    }
}
impl std::error::Error for AsUIntError {}
impl JsonNumber {
    pub fn as_uint(&self) -> Result<usize, AsUIntError> {
        match self {
            JsonNumber::UInt(result) => Ok(*result),
            _ => Err(AsUIntError),
        }
    }
    pub fn as_int(&self) -> Result<isize, AsIntError> {
        match self {
            JsonNumber::Int(result) => Ok(*result),
            _ => Err(AsIntError),
        }
    }
    pub fn as_float(&self) -> Result<f64, AsFloatError> {
        match self {
            JsonNumber::Float(result) => Ok(*result),
            _ => Err(AsFloatError),
        }
    }
    pub fn is_float(&self) -> bool {
        match self {
            JsonNumber::Float(_) => true,
            _ => false,
        }
    }
    pub fn is_int(&self) -> bool {
        match self {
            JsonNumber::Int(_) => true,
            _ => false,
        }
    }
    pub fn is_uint(&self) -> bool {
        match self {
            JsonNumber::UInt(_) => true,
            _ => false,
        }
    }
}

impl From<Value> for Json {
    fn from(json: Value) -> Self {
        match json {
            Value::Null => Json::Null,
            Value::Bool(bool) => Json::Boolean(bool),
            Value::String(str) => Json::String(str),
            Value::Number(num) => {
                if num.is_u64() {
                    return Json::Number(JsonNumber::UInt(num.as_u64().unwrap() as usize));
                }
                if num.is_i64() {
                    return Json::Number(JsonNumber::Int(num.as_i64().unwrap() as isize));
                }
                Json::Number(JsonNumber::Float(num.as_f64().unwrap()))
            }
            Value::Object(obj) => {
                let obj = obj
                    .into_iter()
                    .map(|(k, v)| (k, Json::from(v)))
                    .collect::<BTreeMap<_, _>>();
                Json::Object(obj)
            }
            Value::Array(arr) => {
                let arr = arr.into_iter().map(|v| Json::from(v)).collect::<Vec<_>>();
                Json::Array(arr)
            }
        }
    }
}

//impl From<&str> for Json {
//fn from(source: &str) -> Self {
////let json: Value = serde_json::from_str(source).unwrap();
////match json {
////jso
////}
//}
//}
