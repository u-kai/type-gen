use crate::parts::{property_type::PropertyType, type_name::TypeName};

#[derive(Debug, Clone, PartialEq)]
pub struct AliasTypeStructure {
    name: TypeName,
    property_type: PropertyType,
}

impl AliasTypeStructure {
    pub fn new(name: impl Into<TypeName>, property_type: PropertyType) -> Self {
        Self {
            name: name.into(),
            property_type,
        }
    }
    pub fn type_name(&self) -> &TypeName {
        &self.name
    }
    pub fn property_type(&self) -> &PropertyType {
        &self.property_type
    }
}

#[cfg(test)]
mod test {
    use crate::parts::property_type::property_type_factories::make_usize_type;

    use super::AliasTypeStructure;
    #[test]
    fn test_alias_type_structure() {
        let type_name = "Test";
        let primitive_type = make_usize_type();
        let alias_type = AliasTypeStructure::new(type_name, primitive_type);
        assert_eq!(alias_type.type_name().as_str(), "Test");
        assert_eq!(alias_type.property_type(), &make_usize_type());
    }
}
