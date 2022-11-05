use crate::traits::json_lang_mapper::{optional::OptionalPrimitive, primitive::Primitive};

use super::primitive::RustJsonPrimitiveMapper;

pub struct RustJsonOptionalMapper {
    inner: RustJsonPrimitiveMapper,
}

impl RustJsonOptionalMapper {
    pub fn new() -> Self {
        Self {
            inner: RustJsonPrimitiveMapper::new(),
        }
    }
    fn make_statement(&self, type_: &str) -> String {
        format!("Option<{}>", type_)
    }
}

impl OptionalPrimitive for RustJsonOptionalMapper {
    fn case_bool(&self) -> String {
        self.make_statement(self.inner.case_bool())
    }
    fn case_f64(&self) -> String {
        self.make_statement(self.inner.case_f64())
    }
    fn case_i64(&self) -> String {
        self.make_statement(self.inner.case_i64())
    }
    fn case_null(&self) -> String {
        self.make_statement(self.inner.case_null())
    }
    fn case_string(&self) -> String {
        self.make_statement(self.inner.case_string())
    }
    fn case_u64(&self) -> String {
        self.make_statement(self.inner.case_u64())
    }
    fn case_type(&self, type_key: &str) -> String {
        self.make_statement(type_key)
    }
}

#[cfg(test)]
mod test_rust_optional {
    use super::*;
    #[test]
    fn test_rust_optional() {
        let op = RustJsonOptionalMapper::new();
        assert_eq!(op.case_string(), "Option<String>".to_string());
    }
}
