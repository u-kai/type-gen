use std::collections::HashMap;

use utils::store_fn::get_tuple_key_store;

use crate::types::{property_key::PropertyKey, type_name::TypeName};

pub trait Visibility: From<String> {
    fn to_type_define(&self) -> &'static str;
    fn to_property_define(&self) -> &'static str;
    fn default_type_visibility() -> &'static str;
    fn default_property_visibility() -> &'static str;
}
#[derive(Debug, Clone)]
pub struct VisibilityStore<V: Visibility> {
    all_type_visibility: Option<V>,
    all_property_visibility: Option<V>,
    type_store: HashMap<TypeName, V>,
    property_store: HashMap<(TypeName, PropertyKey), V>,
}

impl<V: Visibility> VisibilityStore<V> {
    pub fn new() -> Self {
        Self {
            all_type_visibility: None,
            all_property_visibility: None,
            type_store: HashMap::new(),
            property_store: HashMap::new(),
        }
    }
    pub fn set_all_type_visibility(&mut self, visibility: V) {
        self.all_type_visibility = Some(visibility)
    }
    pub fn set_all_property_visibility(&mut self, visibility: V) {
        self.all_property_visibility = Some(visibility)
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
        if self.all_type_visibility.is_some() {
            return self
                .all_type_visibility
                .as_ref()
                .map(|c| c.to_type_define())
                .unwrap();
        }
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
        if self.all_property_visibility.is_some() {
            return self
                .all_property_visibility
                .as_ref()
                .map(|c| c.to_property_define())
                .unwrap();
        }
        get_tuple_key_store(&self.property_store, type_name, property_key)
            .map(|v| v.to_property_define())
            .unwrap_or(V::default_property_visibility())
    }
}
