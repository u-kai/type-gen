use super::{primitive_type::PrimitiveType, type_name::TypeName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyType {
    Array(Box<PropertyType>),
    Optional(Box<PropertyType>),
    Primitive(PrimitiveType),
    CustomType(TypeName),
    Any,
}
impl PropertyType {
    pub fn to_optional(self) -> Self {
        Self::Optional(Box::new(self))
    }
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
    pub fn make_any() -> PropertyType {
        PropertyType::Any
    }
}
