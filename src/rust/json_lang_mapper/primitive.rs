use crate::traits::json_lang_mapper::primitive::Primitive;

pub struct RustJsonPrimitiveMapper;
impl RustJsonPrimitiveMapper {
    pub fn new() -> Self {
        Self
    }
}

impl Primitive for RustJsonPrimitiveMapper {
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
}
