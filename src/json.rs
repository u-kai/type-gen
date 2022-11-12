use std::collections::BTreeMap;

use serde_json::{Number, Value};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Json {
    Array(Vec<Json>),
    Boolean(bool),
    Object(BTreeMap<String, Json>),
    Number(Number),
    Null,
    String(String),
}

impl From<Value> for Json {
    fn from(json: Value) -> Self {
        match json {
            Value::Null => Json::Null,
            Value::Bool(bool) => Json::Boolean(bool),
            Value::String(str) => Json::String(str),
            Value::Number(num) => Json::Number(num),
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

impl From<&str> for Json {
    fn from(source: &str) -> Self {
        let json: Value = serde_json::from_str(source).unwrap();
        Json::from(json)
    }
}

#[cfg(test)]
mod test_json {
    use super::Json;
    use std::collections::BTreeMap;
    #[test]
    fn test_from_str_to_json() {
        let source = r#"{"key":"value"}"#;
        let mut map = BTreeMap::new();
        map.insert("key".to_string(), Json::String("value".to_string()));
        let tobe = Json::Object(map);
        assert_eq!(Json::from(source), tobe);
        let source = r#"{"key":{"data":[{"id":"0"},{"id":"1"}]}}"#;
        let mut map = BTreeMap::new();
        let mut child = BTreeMap::new();
        let mut child_child_0 = BTreeMap::new();
        child_child_0.insert("id".to_string(), Json::String("0".to_string()));
        let mut child_child_1 = BTreeMap::new();
        child_child_1.insert("id".to_string(), Json::String("1".to_string()));
        child.insert(
            "data".to_string(),
            Json::Array(vec![
                Json::Object(child_child_0),
                Json::Object(child_child_1),
            ]),
        );
        map.insert("key".to_string(), Json::Object(child));
        let tobe = Json::Object(map);
        assert_eq!(Json::from(source), tobe);
    }
}
