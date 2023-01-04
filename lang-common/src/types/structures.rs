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
        let name = name.into();
        let properties = properties.into_iter().map(|(p, t)| (p.into(), t)).collect();
        Self::Composite(CompositeTypeStructure::new(name, properties))
    }
    pub fn make_alias(name: impl Into<TypeName>, property_type: PropertyType) -> Self {
        Self::Alias(AliasTypeStructure::new(name.into(), property_type))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompositeTypeStructure {
    name: TypeName,
    properties: BTreeMap<PropertyKey, PropertyType>,
}
impl CompositeTypeStructure {
    pub fn new(name: impl Into<TypeName>, properties: BTreeMap<PropertyKey, PropertyType>) -> Self {
        Self {
            name: name.into(),
            properties,
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = (&PropertyKey, &PropertyType)> {
        self.properties.iter()
    }
    pub fn type_name(&self) -> &TypeName {
        &self.name
    }
}

#[cfg(test)]
mod test_composite_type_structure {
    use std::collections::BTreeMap;

    use crate::types::{
        primitive_type::primitive_type_factories::make_usize, property_key::PropertyKey,
        property_type::property_type_factories::make_primitive_type,
    };

    use super::CompositeTypeStructure;

    #[test]
    fn test_into_iter() {
        let properties = vec![("id", make_primitive_type(make_usize()))];
        let properties = properties
            .into_iter()
            .map(|(key, type_)| {
                let key: PropertyKey = key.into();
                (key, type_)
            })
            .collect::<BTreeMap<_, _>>();
        let composite = CompositeTypeStructure::new("Test", properties);
        for (key, type_) in composite.iter() {
            assert_eq!(key.as_str(), "id");
            assert_eq!(type_, &make_primitive_type(make_usize()));
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct AliasTypeStructure {
    pub name: TypeName,
    pub property_type: PropertyType,
}

impl AliasTypeStructure {
    pub fn new(name: impl Into<TypeName>, property_type: PropertyType) -> Self {
        Self {
            name: name.into(),
            property_type,
        }
    }
}
