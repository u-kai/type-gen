pub trait JsonLangMapper {
    fn case_string(&self) -> &'static str;
    fn case_null(&self) -> &'static str;
    fn case_i64(&self) -> &'static str;
    fn case_u64(&self) -> &'static str;
    fn case_f64(&self) -> &'static str;
    fn case_bool(&self) -> &'static str;
    fn make_array_type(&self, type_str: &str) -> String;
    fn make_optional_type(&self, type_str: &str) -> String;
}
