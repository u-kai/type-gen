use std::collections::BTreeMap;

use crate::parts::{property_key::PropertyKey, property_type::PropertyType, type_name::TypeName};

/// CompositeTypeStructure
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
    pub fn type_name(&self) -> &TypeName {
        &self.name
    }
    pub fn iter(&self) -> impl Iterator<Item = (&PropertyKey, &PropertyType)> {
        self.properties.iter()
    }
}

#[cfg(test)]
mod test_composite_type_structure {
    use crate::parts::property_type::property_type_factories::make_usize_type;

    use super::*;

    #[test]
    fn test_into_iter() {
        let properties = vec![("id", make_usize_type())];
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
            assert_eq!(type_, &make_usize_type());
        }
    }
}
