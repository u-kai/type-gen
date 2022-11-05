pub trait OptionalPrimitiveArray {
    fn case_string(&self) -> String;
    fn case_null(&self) -> String;
    fn case_i64(&self) -> String;
    fn case_u64(&self) -> String;
    fn case_f64(&self) -> String;
    fn case_bool(&self) -> String;
    fn case_type(&self, type_key: &str) -> String;
}
