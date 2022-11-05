pub trait Primitive {
    fn case_string(&self) -> &'static str;
    fn case_null(&self) -> &'static str;
    fn case_i64(&self) -> &'static str;
    fn case_u64(&self) -> &'static str;
    fn case_f64(&self) -> &'static str;
    fn case_bool(&self) -> &'static str;
}
