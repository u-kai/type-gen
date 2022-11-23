use std::collections::BTreeMap;

use super::{
    primitive_type::PrimitiveType, property_key::PropertyKey, property_type::PropertyType,
    type_name::TypeName,
};
#[derive(Debug, Clone, PartialEq)]
pub enum TypeStructure {
    Composite(CompositeTypeStructure),
    Primitive(PrimitiveTypeStructure),
}

impl TypeStructure {
    pub fn make_composite(
        name: impl Into<TypeName>,
        properties: Vec<(&str, PropertyType)>,
    ) -> Self {
        Self::Composite(CompositeTypeStructure::new_easy(name, properties))
    }
    pub fn make_primitive(name: impl Into<TypeName>, primitive_type: PrimitiveType) -> Self {
        Self::Primitive(PrimitiveTypeStructure::new(name.into(), primitive_type))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompositeTypeStructure {
    pub name: TypeName,
    pub properties: BTreeMap<PropertyKey, PropertyType>,
}

impl CompositeTypeStructure {
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
pub struct PrimitiveTypeStructure {
    pub name: TypeName,
    pub primitive_type: PrimitiveType,
}

impl PrimitiveTypeStructure {
    pub fn new(name: TypeName, primitive_type: PrimitiveType) -> Self {
        Self {
            name,
            primitive_type,
        }
    }
}
