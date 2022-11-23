use std::collections::HashMap;

use crate::types::{property_key::PropertyKey, type_name::TypeName};

pub trait Visibility {
    fn to_define(&self) -> &'static str;
}
pub struct VisibilityStore<'a, V: Visibility> {
    type_store: HashMap<&'a TypeName, V>,
    property_store: HashMap<(&'a TypeName, &'a PropertyKey), V>,
}

impl<'a, V: Visibility> VisibilityStore<'a, V> {
    pub fn new() -> Self {
        Self {
            type_store: HashMap::new(),
            property_store: HashMap::new(),
        }
    }
    pub fn add_type_visibility(&mut self, type_name: &'a TypeName, visibility: V) {
        self.type_store.insert(type_name, visibility);
    }
    pub fn add_property_visibility(
        &mut self,
        type_name: &'a TypeName,
        property_key: &'a PropertyKey,
        visibility: V,
    ) {
        self.property_store
            .insert((type_name, property_key), visibility);
    }
    pub fn get_type_visibility(&self, type_name: &TypeName) -> Option<&'static str> {
        self.type_store.get(type_name).map(|visi| visi.to_define())
    }
    pub fn get_property_visibility(
        &self,
        type_name: &'a TypeName,
        property_key: &'a PropertyKey,
    ) -> Option<&'static str> {
        self.property_store
            .get(&(type_name, property_key))
            .map(|visi| visi.to_define())
    }
}
