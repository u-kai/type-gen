use std::collections::BTreeMap;

use super::structures::{PrimitiveType, PropertyKey, TypeName};
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

#[derive(Debug, Clone, PartialEq)]
pub enum TypeStatement {
    Composite(CompositeTypeStatement),
    Primitive(PrimitiveTypeStatement),
}

impl TypeStatement {
    pub fn make_composite(
        name: impl Into<TypeName>,
        properties: Vec<(&str, PropertyType)>,
    ) -> Self {
        Self::Composite(CompositeTypeStatement::new_easy(name, properties))
    }
    pub fn make_primitive(name: impl Into<TypeName>, primitive_type: PrimitiveType) -> Self {
        Self::Primitive(PrimitiveTypeStatement::new(name.into(), primitive_type))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompositeTypeStatement {
    pub name: TypeName,
    pub properties: BTreeMap<PropertyKey, PropertyType>,
}

impl CompositeTypeStatement {
    pub fn new(name: impl Into<TypeName>, properties: BTreeMap<PropertyKey, PropertyType>) -> Self {
        Self {
            name: name.into(),
            properties,
        }
    }
    fn new_easy(name: impl Into<TypeName>, properties: Vec<(&str, PropertyType)>) -> Self {
        let name = name.into();
        let properties = properties.into_iter().map(|(p, t)| (p.into(), t)).collect();
        Self { name, properties }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct PrimitiveTypeStatement {
    pub name: TypeName,
    pub primitive_type: PrimitiveType,
}

impl PrimitiveTypeStatement {
    pub fn new(name: TypeName, primitive_type: PrimitiveType) -> Self {
        Self {
            name,
            primitive_type,
        }
    }
}
