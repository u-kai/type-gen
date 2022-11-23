use std::collections::HashMap;

use utils::store_fn::get_tuple_key_store;

use crate::types::{property_key::PropertyKey, type_name::TypeName};

pub trait Attribute {
    fn to_type_define(&self) -> String;
    fn to_property_define(&self) -> String;
}
pub struct AttributeStore<A>
where
    A: Attribute,
{
    type_store: HashMap<TypeName, A>,
    property_store: HashMap<(TypeName, PropertyKey), A>,
}

impl<A> AttributeStore<A>
where
    A: Attribute,
{
    pub fn new() -> Self {
        Self {
            type_store: HashMap::new(),
            property_store: HashMap::new(),
        }
    }
    pub fn add_type_attribute(&mut self, type_name: impl Into<TypeName>, attribute: A) {
        self.type_store.insert(type_name.into(), attribute);
    }
    pub fn add_property_attribute(
        &mut self,
        type_name: impl Into<TypeName>,
        property_key: impl Into<PropertyKey>,
        attribute: A,
    ) {
        self.property_store
            .insert((type_name.into(), property_key.into()), attribute);
    }
    pub fn get_type_attribute(&self, type_name: &TypeName) -> Option<String> {
        self.type_store
            .get(type_name)
            .map(|attr| attr.to_type_define())
    }
    pub fn get_property_attribute(
        &self,
        type_name: &TypeName,
        property_key: &PropertyKey,
    ) -> Option<String> {
        get_tuple_key_store(&self.property_store, type_name, property_key)
            .map(|attr| attr.to_property_define())
    }
}
