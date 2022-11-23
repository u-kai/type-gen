use std::collections::HashMap;

use utils::store_fn::push_to_kv_vec;

use super::structures::{PropertyKey, TypeName};

pub type Attribute = String;
pub struct AttributeStore<'a> {
    type_store: HashMap<&'a TypeName, Vec<Attribute>>,
    property_store: HashMap<(&'a TypeName, &'a PropertyKey), Vec<Attribute>>,
}

impl<'a> AttributeStore<'a> {
    pub fn new() -> Self {
        Self {
            type_store: HashMap::new(),
            property_store: HashMap::new(),
        }
    }
    pub fn add_type_attribute(&mut self, type_name: &'a TypeName, attribute: Attribute) {
        push_to_kv_vec(&mut self.type_store, type_name, attribute);
    }
    pub fn add_property_attribute(
        &mut self,
        type_name: &'a TypeName,
        property_key: &'a PropertyKey,
        attribute: Attribute,
    ) {
        push_to_kv_vec(
            &mut self.property_store,
            (type_name, property_key),
            attribute,
        );
    }
    pub fn get_type_attribute(&self, type_name: &TypeName) -> Option<&Vec<Attribute>> {
        self.type_store.get(type_name)
    }
    pub fn get_property_attribute(
        &self,
        type_name: &'a TypeName,
        property_key: &'a PropertyKey,
    ) -> Option<&Vec<Attribute>> {
        self.property_store.get(&(type_name, property_key))
    }
}
