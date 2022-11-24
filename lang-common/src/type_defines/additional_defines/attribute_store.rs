use std::collections::HashMap;

use utils::store_fn::get_tuple_key_store;

use crate::types::{property_key::PropertyKey, type_name::TypeName};

pub trait Attribute: From<String> {
    fn to_type_define(&self) -> String;
    fn to_property_define(&self) -> String;
}
///impl<A,I> From<I> for A
///where
///A:Attribute,
///I:Into<String>
///{
///fn from(str: I) -> Self {
///str.into()
///}
///}
pub struct AttributeStore<A>
where
    A: Attribute,
{
    all_type_attribute: Option<A>,
    all_property_attribute: Option<A>,
    type_store: HashMap<TypeName, A>,
    property_store: HashMap<(TypeName, PropertyKey), A>,
}

impl<A> AttributeStore<A>
where
    A: Attribute,
{
    pub fn new() -> Self {
        Self {
            all_type_attribute: None,
            all_property_attribute: None,
            type_store: HashMap::new(),
            property_store: HashMap::new(),
        }
    }
    pub fn set_all_type_attribute(&mut self, attribute: A) {
        self.all_type_attribute = Some(attribute)
    }
    pub fn set_all_property_attribute(&mut self, attribute: A) {
        self.all_property_attribute = Some(attribute)
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
        if self.all_type_attribute.is_some() {
            return self.all_type_attribute.as_ref().map(|a| a.to_type_define());
        }
        self.type_store
            .get(type_name)
            .map(|attr| attr.to_type_define())
    }
    pub fn get_property_attribute(
        &self,
        type_name: &TypeName,
        property_key: &PropertyKey,
    ) -> Option<String> {
        if self.all_property_attribute.is_some() {
            return self
                .all_property_attribute
                .as_ref()
                .map(|a| a.to_property_define());
        }
        get_tuple_key_store(&self.property_store, type_name, property_key)
            .map(|attr| attr.to_property_define())
    }
}
