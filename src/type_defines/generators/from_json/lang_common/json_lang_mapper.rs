use serde_json::Number;

pub trait JsonLangMapper {
    fn case_string(&self) -> &'static str;
    fn case_null(&self) -> &'static str;
    fn case_bool(&self) -> &'static str;
    fn case_num(&self, num: &Number) -> String;
    fn case_any(&self) -> &'static str;
    fn make_array_type(&self, type_str: &str) -> String;
    fn make_optional_type(&self, type_str: &str) -> String;
}
