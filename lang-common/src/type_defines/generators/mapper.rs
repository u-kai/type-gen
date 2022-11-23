use crate::types::{
    statement::PropertyType,
    structures::{Number, PrimitiveType},
};

pub type TypeString = String;
pub trait LangTypeMapper {
    fn case_string(&self) -> TypeString;
    fn case_boolean(&self) -> TypeString;
    fn case_usize(&self) -> TypeString;
    fn case_isize(&self) -> TypeString;
    fn case_float(&self) -> TypeString;
    fn case_null(&self) -> TypeString;
    fn case_any(&self) -> TypeString;
    fn case_array_type<T: Into<TypeString>>(&self, type_statement: T) -> TypeString;
    fn case_optional_type<T: Into<TypeString>>(&self, type_statement: T) -> TypeString;
    fn case_primitive(&self, primitive_type: &PrimitiveType) -> TypeString {
        match primitive_type {
            PrimitiveType::Boolean => self.case_boolean(),
            PrimitiveType::String => self.case_string(),
            PrimitiveType::Number(num) => match num {
                Number::Float => self.case_float(),
                Number::Usize => self.case_usize(),
                Number::Isize => self.case_isize(),
            },
        }
    }
    fn case_property_type(&self, property_type: &PropertyType) -> TypeString {
        match property_type {
            PropertyType::Any => self.case_any(),
            PropertyType::Primitive(primitive) => self.case_primitive(primitive),
            PropertyType::Optional(optional_type) => {
                self.case_optional_type(self.case_property_type(optional_type))
            }
            PropertyType::Array(array_type) => {
                self.case_array_type(self.case_property_type(array_type))
            }
            PropertyType::CustomType(custom_type) => custom_type.as_str().to_string(),
        }
    }
}
