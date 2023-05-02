use description_generator::type_mapper::{TypeMapper, TypeString};
use structure::parts::type_name::TypeName;

pub struct GoMapper;

impl TypeMapper for GoMapper {
    fn case_string(&self) -> TypeString {
        "string".to_string()
    }
    fn case_null(&self) -> TypeString {
        self.case_any()
    }
    fn case_custom_type(&self, custom_type: &TypeName) -> String {
        custom_type.valid_lang_str()
    }
    fn case_any(&self) -> TypeString {
        "interface {}".to_string()
    }
    fn case_boolean(&self) -> TypeString {
        "bool".to_string()
    }
    fn case_array_type<T: Into<TypeString>>(&self, type_statement: T) -> TypeString {
        format!("[]{}", type_statement.into())
    }
    fn case_optional_type<T: Into<TypeString>>(&self, _type_statement: T) -> TypeString {
        todo!("go is not defined optional")
        //format!("", type_statement.into())
    }
    fn case_float(&self) -> TypeString {
        "float64".to_string()
    }
    fn case_isize(&self) -> TypeString {
        "int".to_string()
    }
    fn case_usize(&self) -> TypeString {
        "int".to_string()
    }
}
