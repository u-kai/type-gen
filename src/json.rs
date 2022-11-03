use std::collections::BTreeMap;

pub enum Json {
    Array(Vec<Json>),
    Boolean(bool),
    Object(BTreeMap<String, Json>),
    Number(f64),
    Null,
    String(String),
}
