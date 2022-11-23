use std::collections::HashMap;

use utils::store_fn::push_to_kv_vec;

use crate::types::{property_key::PropertyKey, type_name::TypeName};

pub trait Comment {
    fn to_define(&self) -> String;
}
pub struct CommentStore<'a, C>
where
    C: Comment,
{
    type_store: HashMap<&'a TypeName, Vec<C>>,
    property_store: HashMap<(&'a TypeName, &'a PropertyKey), Vec<C>>,
}

impl<'a, C> CommentStore<'a, C>
where
    C: Comment,
{
    pub fn new() -> Self {
        Self {
            type_store: HashMap::new(),
            property_store: HashMap::new(),
        }
    }
    pub fn add_type_comment(&mut self, type_name: &'a TypeName, attribute: C) {
        push_to_kv_vec(&mut self.type_store, type_name, attribute);
    }
    pub fn add_property_comment(
        &mut self,
        type_name: &'a TypeName,
        property_key: &'a PropertyKey,
        comment: C,
    ) {
        push_to_kv_vec(&mut self.property_store, (type_name, property_key), comment);
    }
    pub fn get_type_comment(&self, type_name: &TypeName) -> Option<&Vec<C>> {
        self.type_store.get(type_name)
    }
    pub fn get_property_comment(
        &self,
        type_name: &'a TypeName,
        property_key: &'a PropertyKey,
    ) -> Option<&Vec<C>> {
        self.property_store.get(&(type_name, property_key))
    }
}
