use std::collections::BTreeMap;

use serde_json::Value;
use utils::store_fn::push_to_btree_vec;

#[derive(Debug, PartialEq, Clone)]
pub enum Json {
    Array(Vec<Json>),
    Boolean(bool),
    Object(BTreeMap<String, Json>),
    Number(Number),
    Null,
    String(String),
}
impl Json {
    pub fn put_together_array_json(array: Vec<Json>) -> Self {
        if array.len() == 0 {
            return Json::Null;
        }
        let rep = &array[0];
        match rep {
            Json::Object(_) => {
                let map = Self::collect_obj_from_array_json(array);
                println!("{:#?}", map);
                let json = Json::Object(map.into_iter().fold(
                    BTreeMap::new(),
                    |mut acc, (key, collected_json)| {
                        acc.insert(key, Self::put_together_array_json(collected_json));
                        acc
                    },
                ));
                json
            }
            Json::Array(_) => {
                let array_flated = array
                    .into_iter()
                    .map(|json| {
                        let Json::Array(v) = json else {
                        panic!("array index 0 is array. but array content is not array {:#?}",json);
                    };
                        Self::put_together_array_json(v)
                    })
                    .collect::<Vec<_>>();
                Json::Array(array_flated)
            }
            Json::Null => Json::Null,
            Json::Number(num) => {
                if num.is_f64() {
                    return Json::Number(Number::Float64(f64::default()));
                }
                if num.is_u64() {
                    return Json::Number(Number::Usize64(u64::default()));
                }
                Json::Number(Number::Isize64(i64::default()))
            }
            Json::String(_) => Json::String(String::default()),
            Json::Boolean(_) => Json::Boolean(bool::default()),
        }
    }
    fn collect_obj_from_array_json(array: Vec<Json>) -> BTreeMap<String, Vec<Json>> {
        fn rec(map: &mut BTreeMap<String, Vec<Json>>, array: Vec<Json>) {
            for json in array {
                match json {
                    Json::Object(obj) => {
                        for (k, v) in obj {
                            push_to_btree_vec(map, k, v);
                        }
                    }
                    Json::Array(array) => rec(map, array),
                    _ => {}
                }
            }
        }
        let mut map = BTreeMap::new();
        rec(&mut map, array);
        map
    }
}
#[cfg(test)]
mod test_from_array_json {
    use super::*;
    #[test]
    fn test_case_nest_obj_array() {
        let obj1 = r#"
        {
            "id":0,
            "name":"kai",
            "data": {
                "age":26
            }
        }
        "#;
        let obj1 = Json::from(obj1);
        let obj2 = r#"
        {
            "id":0,
            "age":25
        }
        "#;
        let obj2 = Json::from(obj2);
        let obj3 = r#"
        {
            "id":0,
            "age":35,
            "data": {
                "age":26,
                "from":"kanagawa"
            }
        }
        "#;
        let obj3 = Json::from(obj3);
        let array_json = vec![Json::Array(vec![obj1, obj2, obj3])];
        let mut data = BTreeMap::new();
        data.insert("from".to_string(), Json::String(String::default()));
        data.insert("age".to_string(), Json::Number(Number::Usize64(0)));
        let mut tobe = BTreeMap::new();
        tobe.insert("id".to_string(), Json::Number(Number::Usize64(0)));
        tobe.insert("age".to_string(), Json::Number(Number::Usize64(0)));
        tobe.insert("name".to_string(), Json::String(String::default()));
        tobe.insert("data".to_string(), Json::Object(data));
        let tobe = Json::Array(vec![Json::Object(tobe)]);
        assert_eq!(Json::put_together_array_json(array_json), tobe)
    }
    #[test]
    fn test_case_obj_array() {
        let obj1 = r#"
        {
            "id":0,
            "name":"kai"
        }
        "#;
        let obj1 = Json::from(obj1);
        let obj2 = r#"
        {
            "id":0,
            "age":25
        }
        "#;
        let obj2 = Json::from(obj2);
        let array_json = vec![Json::Array(vec![obj1, obj2])];
        let mut tobe = BTreeMap::new();
        tobe.insert("id".to_string(), Json::Number(Number::Usize64(0)));
        tobe.insert("age".to_string(), Json::Number(Number::Usize64(0)));
        tobe.insert("name".to_string(), Json::String(String::default()));
        let tobe = Json::Array(vec![Json::Object(tobe)]);
        assert_eq!(Json::put_together_array_json(array_json), tobe)
    }
    #[test]
    fn test_case_nest_primitive_array() {
        let array_json = vec![Json::Array(vec![
            Json::String("value1".to_string()),
            Json::String("value2".to_string()),
        ])];
        let tobe = Json::Array(vec![Json::String(String::default())]);
        assert_eq!(Json::put_together_array_json(array_json), tobe)
    }
    #[test]
    fn test_case_primitive_array() {
        let array_json = vec![
            Json::String("value1".to_string()),
            Json::String("value2".to_string()),
        ];
        let tobe = Json::String(String::default());
        assert_eq!(Json::put_together_array_json(array_json), tobe)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Float64(f64),
    Usize64(u64),
    Isize64(i64),
}
impl Number {
    pub fn is_f64(&self) -> bool {
        match self {
            Self::Float64(_) => true,
            _ => false,
        }
    }
    pub fn is_u64(&self) -> bool {
        match self {
            Self::Usize64(_) => true,
            _ => false,
        }
    }
    pub fn is_i64(&self) -> bool {
        match self {
            Self::Isize64(_) => true,
            _ => false,
        }
    }
}

impl From<u64> for Number {
    fn from(i: u64) -> Self {
        Number::Usize64(i)
    }
}
impl From<i64> for Number {
    fn from(i: i64) -> Self {
        Number::Isize64(i)
    }
}
impl From<f64> for Number {
    fn from(i: f64) -> Self {
        Number::Float64(i)
    }
}
impl From<serde_json::Number> for Number {
    fn from(num: serde_json::Number) -> Self {
        if num.is_f64() {
            return Number::from(num.as_f64().unwrap());
        }
        if num.is_u64() {
            return Number::Usize64(num.as_u64().unwrap());
        }
        Number::from(num.as_i64().unwrap())
    }
}
impl From<Value> for Json {
    fn from(json: Value) -> Self {
        match json {
            Value::Null => Json::Null,
            Value::Bool(bool) => Json::Boolean(bool),
            Value::String(str) => Json::String(str),
            Value::Number(num) => Json::Number(Number::from(num)),
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
