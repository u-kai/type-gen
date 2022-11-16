use crate::type_defines::generators::from_json::lang_common::json_lang_mapper::JsonLangMapper;

pub struct JsonRustMapper;
impl JsonRustMapper {
    pub fn new() -> Self {
        Self
    }
}

impl JsonLangMapper for JsonRustMapper {
    fn case_string(&self) -> &'static str {
        "String"
    }
    fn case_null(&self) -> &'static str {
        self.case_any()
    }
    fn case_num(&self, num: &serde_json::Number) -> String {
        if num.is_f64() {
            return "f64".to_string();
        }
        if num.is_i64() {
            return "i64".to_string();
        }
        "u64".to_string()
    }
    fn case_any(&self) -> &'static str {
        "serde_json::Value"
    }
    fn case_bool(&self) -> &'static str {
        "bool"
    }
    fn make_array_type(&self, type_str: &str) -> String {
        format!("Vec<{}>", type_str)
    }
    fn make_optional_type(&self, type_str: &str) -> String {
        format!("Option<{}>", type_str)
    }
}
