use std::collections::BTreeMap;

use serde_json::Number;

use crate::json::Json;

pub trait JsonTypeConvertor {
    fn case_null(&self) -> String;
    fn case_boolean(&self, bool: bool) -> String;
    fn case_number(&self, num: &Number) -> String;
    fn case_string(&self, str: &str) -> String;
    fn case_array(&self, arr: &Vec<Json>, stack: &mut Vec<String>) -> String;
    fn case_object(&self, obj: &BTreeMap<String, Json>, stack: &mut Vec<String>) -> String;
    fn case_null_with_key(&self, key: &str) -> String;
    fn case_boolean_with_key(&self, key: &str, bool: bool) -> String;
    fn case_number_with_key(&self, key: &str, num: &Number) -> String;
    fn case_string_with_key(&self, key: &str, str: &str) -> String;
    fn case_array_with_key(&self, key: &str, arr: &Vec<Json>, stack: &mut Vec<String>) -> String;
    fn case_object_with_key(
        &self,
        key: &str,
        obj: &BTreeMap<String, Json>,
        stack: &mut Vec<String>,
    ) -> String;
}
