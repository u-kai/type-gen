use crate::traits::json_lang_mapper::JsonLangMapper;

pub struct JsonRustMapper;
impl JsonRustMapper {
    pub fn new() -> Self {
        Self
    }
}

impl JsonLangMapper for JsonRustMapper {
    fn case_u64(&self) -> &'static str {
        "u64"
    }
    fn case_string(&self) -> &'static str {
        "String"
    }
    fn case_null(&self) -> &'static str {
        "String"
    }
    fn case_i64(&self) -> &'static str {
        "i64"
    }
    fn case_f64(&self) -> &'static str {
        "f64"
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
