use description_generator::type_mapper::{TypeMapper, TypeString};
use structure::parts::type_name::TypeName;

pub struct RustMapper;

impl TypeMapper for RustMapper {
    fn case_string(&self) -> TypeString {
        "String".to_string()
    }
    fn case_null(&self) -> TypeString {
        self.case_any()
    }
    fn case_custom_type(&self, custom_type: &TypeName) -> String {
        custom_type.valid_lang_str()
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
//fn replace_cannot_use_char(str: &str) -> String {
//str.replace(cannot_use_char, "")
//}
//fn cannot_use_char(c: char) -> bool {
//match c {
//':' | ';' | '#' | '$' | '%' | '&' | '~' | '=' | '|' | '\"' | '\'' | '{' | '}' | '?'
//| '!' | '<' | '>' | '[' | ']' | '*' | '^' => true,
//_ => false,
//}
//}
