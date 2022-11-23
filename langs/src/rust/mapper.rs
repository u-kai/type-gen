use lang_common::type_defines::generators::mapper::{LangTypeMapper, TypeString};

pub struct RustLangMapper;

impl LangTypeMapper for RustLangMapper {
    fn case_string(&self) -> TypeString {
        "String".to_string()
    }
    fn case_null(&self) -> TypeString {
        self.case_any()
    }

    fn case_any(&self) -> TypeString {
        "serde_json::Value".to_string()
    }
    fn case_boolean(&self) -> TypeString {
        "bool".to_string()
    }
    fn case_array_type<T: Into<TypeString>>(&self, type_statement: T) -> TypeString {
        format!("Vec<{}>", type_statement.into())
    }
    fn case_optional_type<T: Into<TypeString>>(&self, type_statement: T) -> TypeString {
        format!("Option<{}>", type_statement.into())
    }
    fn case_float(&self) -> TypeString {
        "f64".to_string()
    }
    fn case_isize(&self) -> TypeString {
        "isize".to_string()
    }
    fn case_usize(&self) -> TypeString {
        "usize".to_string()
    }
}
