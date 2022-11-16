use std::collections::HashMap;

use crate::{
    type_defines::{
        generators::from_json::lang_common::field_statements::field_attr::FieldAttribute,
        statement_parts::{field_key::Fieldkey, type_key::TypeKey},
    },
    utils::store_fn::push_to_kv_vec,
};

#[derive(Debug, Clone)]
pub enum RustFieldAttribute {
    CfgTest,
    Original(String),
}
impl RustFieldAttribute {
    pub fn vec_to_string(v: Vec<Self>) -> String {
        let attrs = v
            .iter()
            .map(|attr| attr.into())
            .collect::<Vec<String>>()
            .join(",");
        format!("#[{}]", attrs)
    }
}
impl Into<String> for RustFieldAttribute {
    fn into(self) -> String {
        match self {
            Self::CfgTest => "cfg(test)".to_string(),
            Self::Original(s) => s,
        }
    }
}
impl From<&RustFieldAttribute> for String {
    fn from(attr: &RustFieldAttribute) -> Self {
        match attr {
            RustFieldAttribute::CfgTest => "cfg(test)".to_string(),
            RustFieldAttribute::Original(s) => s.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RustFieldAttributeStore<'a> {
    all: Option<Vec<RustFieldAttribute>>,
    store: HashMap<(&'a str, &'a str), Vec<RustFieldAttribute>>,
}

impl<'a> RustFieldAttributeStore<'a> {
    pub fn new() -> Self {
        Self {
            all: None,
            store: HashMap::new(),
        }
    }
    pub fn set_attr_all(&mut self, attrs: Vec<RustFieldAttribute>) {
        self.all = Some(attrs)
    }
    pub fn add_attr(
        &mut self,
        type_key: &'a TypeKey,
        field_key: &'a Fieldkey,
        attr: RustFieldAttribute,
    ) {
        push_to_kv_vec(
            &mut self.store,
            (type_key.value(), field_key.original()),
            attr,
        )
    }
    pub fn containe(&self, type_key: &str, field_key: &str) -> bool {
        self.store.contains_key(&(type_key, field_key))
    }
}

impl<'a> FieldAttribute for RustFieldAttributeStore<'a> {
    fn get_attr(&self, type_key: &TypeKey, field_key: &Fieldkey) -> Option<String> {
        let mut v = Vec::new();
        if let Some(all) = &self.all {
            v.extend(all.clone());
        }
        self.store
            .get(&(type_key.value(), field_key.original()))
            .map(|attr| {
                v.extend(attr.clone());
            });
        if v.len() > 0 {
            return Some(RustFieldAttribute::vec_to_string(v));
        }
        None
    }
}
