use std::collections::HashMap;

use utils::store_fn::get_tuple_key_store;

use crate::types::{property_key::PropertyKey, type_name::TypeName};

pub trait Comment: From<String> {
    fn to_type_define(&self) -> String;
    fn to_property_define(&self) -> String;
}
pub struct CommentStore<C>
where
    C: Comment,
{
    all_type_comment: Option<C>,
    all_property_comment: Option<C>,
    type_store: HashMap<TypeName, C>,
    property_store: HashMap<(TypeName, PropertyKey), C>,
}

impl<C> CommentStore<C>
where
    C: Comment,
{
    pub fn new() -> Self {
        Self {
            all_type_comment: None,
            all_property_comment: None,
            type_store: HashMap::new(),
            property_store: HashMap::new(),
        }
    }
    pub fn set_all_type_comment(&mut self, comment: C) {
        self.all_type_comment = Some(comment)
    }
    pub fn set_all_property_comment(&mut self, comment: C) {
        self.all_property_comment = Some(comment)
    }
    pub fn add_type_comment(&mut self, type_name: impl Into<TypeName>, comment: C) {
        self.type_store.insert(type_name.into(), comment);
    }
    pub fn add_property_comment(
        &mut self,
        type_name: impl Into<TypeName>,
        property_key: impl Into<PropertyKey>,
        comment: C,
    ) {
        self.property_store
            .insert((type_name.into(), property_key.into()), comment);
    }
    pub fn get_type_comment(&self, type_name: &TypeName) -> Option<String> {
        if self.all_type_comment.is_some() {
            return self.all_type_comment.as_ref().map(|c| c.to_type_define());
        }
        self.type_store
            .get(type_name)
            .map(|comment| comment.to_type_define())
    }
    pub fn get_property_comment(
        &self,
        type_name: &TypeName,
        property_key: &PropertyKey,
    ) -> Option<String> {
        if self.all_property_comment.is_some() {
            return self
                .all_property_comment
                .as_ref()
                .map(|c| c.to_property_define());
        }
        get_tuple_key_store(&self.property_store, type_name, property_key)
            .map(|comment| comment.to_property_define())
    }
}
