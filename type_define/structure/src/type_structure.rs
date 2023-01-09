use crate::{
    alias_type_structure::AliasTypeStructure,
    composite_type_structure::CompositeTypeStructure,
    parts::{property_type::PropertyType, type_name::TypeName},
};

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
        let name = name.into();
        let properties = properties.into_iter().map(|(p, t)| (p.into(), t)).collect();
        Self::Composite(CompositeTypeStructure::new(name, properties))
    }
    pub fn make_alias(name: impl Into<TypeName>, property_type: PropertyType) -> Self {
        Self::Alias(AliasTypeStructure::new(name.into(), property_type))
    }
}
