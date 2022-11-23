use std::collections::HashMap;

use crate::types::{property_key::PropertyKey, type_name::TypeName};

pub trait Comment {
    fn to_define(&self) -> String;
}
pub struct CommentStore<'a, C>
where
    C: Comment,
{
    type_store: HashMap<&'a TypeName, C>,
    property_store: HashMap<(&'a TypeName, &'a PropertyKey), C>,
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
    pub fn add_type_comment(&mut self, type_name: &'a TypeName, comment: C) {
        self.type_store.insert(type_name, comment);
    }
    pub fn add_property_comment(
        &mut self,
        type_name: &'a TypeName,
        property_key: &'a PropertyKey,
        comment: C,
    ) {
        self.property_store
            .insert((type_name, property_key), comment);
    }
    pub fn get_type_comment(&self, type_name: &TypeName) -> Option<String> {
        self.type_store
            .get(type_name)
            .map(|comment| comment.to_define())
    }
    pub fn get_property_comment(
        &self,
        type_name: &'a TypeName,
        property_key: &'a PropertyKey,
    ) -> Option<String> {
        self.property_store
            .get(&(type_name, property_key))
            .map(|comment| comment.to_define())
    }
}
