use std::collections::HashMap;

use utils::store_fn::get_tuple_key_store;

use crate::types::{property_key::PropertyKey, type_name::TypeName};

pub trait Visibility {
    fn to_type_define(&self) -> &'static str;
    fn to_property_define(&self) -> &'static str;
    fn default_type_visibility() -> &'static str;
    fn default_property_visibility() -> &'static str;
}
pub struct VisibilityStore<V: Visibility> {
    type_store: HashMap<TypeName, V>,
    property_store: HashMap<(TypeName, PropertyKey), V>,
}

impl<V: Visibility> VisibilityStore<V> {
    pub fn new() -> Self {
        Self {
            type_store: HashMap::new(),
            property_store: HashMap::new(),
        }
    }
    pub fn add_type_visibility(&mut self, type_name: impl Into<TypeName>, visibility: V) {
        self.type_store.insert(type_name.into(), visibility);
    }
    pub fn add_property_visibility(
        &mut self,
        type_name: impl Into<TypeName>,
        property_key: impl Into<PropertyKey>,
        visibility: V,
    ) {
        self.property_store
            .insert((type_name.into(), property_key.into()), visibility);
    }
    pub fn get_type_visibility(&self, type_name: &TypeName) -> &'static str {
        self.type_store
            .get(type_name)
            .map(|visi| visi.to_type_define())
            .unwrap_or(V::default_type_visibility())
    }
    pub fn get_property_visibility(
        &self,
        type_name: &TypeName,
        property_key: &PropertyKey,
    ) -> &'static str {
        get_tuple_key_store(&self.property_store, type_name, property_key)
            .map(|v| v.to_property_define())
            .unwrap_or(V::default_property_visibility())
    }
}
