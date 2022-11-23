use std::collections::HashMap;

use crate::types::{property_key::PropertyKey, type_name::TypeName};

pub trait Attribute {
    fn to_type_define(&self) -> String;
    fn to_property_define(&self) -> String;
}
pub struct AttributeStore<'a, A>
where
    A: Attribute,
{
    type_store: HashMap<&'a TypeName, A>,
    property_store: HashMap<(&'a TypeName, &'a PropertyKey), A>,
}

impl<'a, A> AttributeStore<'a, A>
where
    A: Attribute,
{
    pub fn new() -> Self {
        Self {
            type_store: HashMap::new(),
            property_store: HashMap::new(),
        }
    }
    pub fn add_type_attribute(&mut self, type_name: &'a TypeName, attribute: A) {
        self.type_store.insert(type_name, attribute);
    }
    pub fn add_property_attribute(
        &mut self,
        type_name: &'a TypeName,
        property_key: &'a PropertyKey,
        attribute: A,
    ) {
        self.property_store
            .insert((type_name, property_key), attribute);
    }
    pub fn get_type_attribute(&self, type_name: &TypeName) -> Option<String> {
        self.type_store
            .get(type_name)
            .map(|attr| attr.to_type_define())
    }
    pub fn get_property_attribute(
        &self,
        type_name: &'a TypeName,
        property_key: &'a PropertyKey,
    ) -> Option<String> {
        self.property_store
            .get(&(type_name, property_key))
            .map(|attr| attr.to_property_define())
    }
}
