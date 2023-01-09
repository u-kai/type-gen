use super::type_name::TypeName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyType {
    Array(Box<PropertyType>),
    Optional(Box<PropertyType>),
    Primitive(PrimitiveType),
    CustomType(TypeName),
    Any,
}
impl PropertyType {
    pub fn new_custom_type(type_name: impl Into<TypeName>) -> Self {
        Self::CustomType(type_name.into())
    }
    pub fn to_optional(self) -> Self {
        Self::Optional(Box::new(self))
    }
    pub fn to_array(self) -> Self {
        Self::Array(Box::new(self))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrimitiveType {
    String,
    Boolean,
    Number(Number),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Number {
    Usize,
    Isize,
    Float,
}

pub mod property_type_factories {
    use super::*;
    pub fn make_array_type(property_type: PropertyType) -> PropertyType {
        PropertyType::Array(Box::new(property_type))
    }
    pub fn make_optional_type(property_type: PropertyType) -> PropertyType {
        PropertyType::Optional(Box::new(property_type))
    }
    pub fn make_custom_type(type_name: impl Into<TypeName>) -> PropertyType {
        PropertyType::CustomType(type_name.into())
    }
    pub fn make_primitive_type(primitive_type: PrimitiveType) -> PropertyType {
        PropertyType::Primitive(primitive_type)
    }
    pub fn make_string_type() -> PropertyType {
        make_primitive_type(make_string())
    }
    #[allow(unused)]
    pub fn make_bool_type() -> PropertyType {
        make_primitive_type(make_bool())
    }
    pub fn make_usize_type() -> PropertyType {
        make_primitive_type(make_usize())
    }
    #[allow(unused)]
    pub fn make_isize_type() -> PropertyType {
        make_primitive_type(make_isize())
    }
    #[allow(unused)]
    pub fn make_float_type() -> PropertyType {
        make_primitive_type(make_float())
    }
    pub fn make_any() -> PropertyType {
        PropertyType::Any
    }
    fn make_string() -> PrimitiveType {
        PrimitiveType::String
    }
    fn make_bool() -> PrimitiveType {
        PrimitiveType::Boolean
    }
    fn make_usize() -> PrimitiveType {
        PrimitiveType::Number(Number::Usize)
    }
    fn make_isize() -> PrimitiveType {
        PrimitiveType::Number(Number::Isize)
    }
    fn make_float() -> PrimitiveType {
        PrimitiveType::Number(Number::Float)
    }
}
