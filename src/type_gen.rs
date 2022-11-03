use crate::{convertor::JsonTypeConvertor, json::Json};

pub struct TypeGenerator<T: JsonTypeConvertor> {
    inner: T,
}
impl<T: JsonTypeConvertor> TypeGenerator<T> {
    pub fn new(convertor: T) -> Self {
        Self { inner: convertor }
    }
    pub fn gen_from_json_example(&self, json: &str) -> String {
        let json = Json::from(json);
        match &json {
            Json::Null => self.inner.case_null(),
            Json::Number(num) => self.inner.case_number(num),
            Json::Array(arr) => self.inner.case_array(arr),
            Json::Boolean(bool) => self.inner.case_boolean(*bool),
            Json::String(str) => self.inner.case_string(str),
            Json::Object(obj) => self.inner.case_object(obj),
        }
    }
}

//#[cfg(test)]
//mod test_type_gen {
//use super::*;
//struct StubJsonTypeConvertor {
//json: Json,
//}
//impl JsonTypeConvertor for StubJsonTypeConvertor {
//fn case_string_with_key(&self, key: &str, str: &str) -> String {
//String::new()
//}
//fn case_string(&self, str: &str) -> String {
//String::new()
//}
//fn case_object_with_key(
//&self,
//key: &str,
//obj: &std::collections::BTreeMap<String, Json>,
//) -> String {
//String::new()
//}
//fn case_object(&self, obj: &std::collections::BTreeMap<String, Json>) -> String {
//String::new()
//}
//fn case_number_with_key(&self, key: &str, num: &serde_json::Number) -> String {
//String::new()
//}
//fn case_number(&self, num: &serde_json::Number) -> String {
//String::new()
//}
//fn case_null_with_key(&self, key: &str) -> String {
//String::new()
//}
//fn case_null(&self) -> String {
//String::new()
//}
//fn case_array(&self, arr: &Vec<Json>) -> String {
//String::new()
//}
//fn case_array_with_key(&self, key: &str, arr: &Vec<Json>) -> String {
//String::new()
//}
//fn case_boolean(&self, bool: bool) -> String {}
//fn case_boolean_with_key(&self, key: &str, bool: bool) -> String {}
//}
//#[test]
//fn test_gen_from_json_example() {}
//}
