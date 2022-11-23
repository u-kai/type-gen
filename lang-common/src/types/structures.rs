use std::collections::BTreeMap;

use super::{property_key::PropertyKey, property_type::PropertyType, type_name::TypeName};
#[derive(Debug, Clone, PartialEq)]
pub enum TypeStructure {
    Composite(CompositeTypeStructure),
    Alias(AliasTypeStructure),
}

impl TypeStructure {
    pub fn make_composite(
        name: impl Into<TypeName>,
        properties: Vec<(&str, PropertyType)>,
    ) -> Self {
        Self::Composite(CompositeTypeStructure::new_easy(name, properties))
    }
    pub fn make_primitive(name: impl Into<TypeName>, property_type: PropertyType) -> Self {
        Self::Alias(AliasTypeStructure::new(name.into(), property_type))
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
pub struct AliasTypeStructure {
    pub name: TypeName,
    pub property_type: PropertyType,
}

impl AliasTypeStructure {
    pub fn new(name: TypeName, property_type: PropertyType) -> Self {
        Self {
            name,
            property_type,
        }
    }
}
