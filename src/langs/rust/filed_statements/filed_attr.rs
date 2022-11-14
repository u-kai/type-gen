use std::collections::HashMap;

use crate::{
    langs::common::type_define_generators::{filed_key::FiledKey, type_key::TypeKey},
    traits::filed_statements::filed_attr::FiledAttribute,
    utils::store_fn::push_to_kv_vec,
};

#[derive(Debug, Clone)]
pub enum RustFiledAttribute {
    CfgTest,
    Original(String),
}
impl RustFiledAttribute {
    pub fn vec_to_string(v: Vec<Self>) -> String {
        let attrs = v
            .iter()
            .map(|attr| attr.into())
            .collect::<Vec<String>>()
            .join(",");
        format!("#[{}]", attrs)
    }
}
impl Into<String> for RustFiledAttribute {
    fn into(self) -> String {
        match self {
            Self::CfgTest => "cfg(test)".to_string(),
            Self::Original(s) => s,
        }
    }
}
impl From<&RustFiledAttribute> for String {
    fn from(attr: &RustFiledAttribute) -> Self {
        match attr {
            RustFiledAttribute::CfgTest => "cfg(test)".to_string(),
            RustFiledAttribute::Original(s) => s.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RustFiledAttributeStore<'a> {
    all: Option<Vec<RustFiledAttribute>>,
    store: HashMap<(&'a str, &'a str), Vec<RustFiledAttribute>>,
}

impl<'a> RustFiledAttributeStore<'a> {
    pub fn new() -> Self {
        Self {
            all: None,
            store: HashMap::new(),
        }
    }
    pub fn set_attr_all(&mut self, attrs: Vec<RustFiledAttribute>) {
        self.all = Some(attrs)
    }
    pub fn add_attr(
        &mut self,
        type_key: &'a TypeKey,
        filed_key: &'a FiledKey,
        attr: RustFiledAttribute,
    ) {
        push_to_kv_vec(&mut self.store, (type_key.value(), filed_key.value()), attr)
    }
    pub fn containe(&self, type_key: &str, filed_key: &str) -> bool {
        self.store.contains_key(&(type_key, filed_key))
    }
}

impl<'a> FiledAttribute for RustFiledAttributeStore<'a> {
    fn get_attr(&self, type_key: &TypeKey, filed_key: &FiledKey) -> Option<String> {
        let mut v = Vec::new();
        if let Some(all) = &self.all {
            v.extend(all.clone());
        }
        self.store
            .get(&(type_key.value(), filed_key.value()))
            .map(|attr| {
                v.extend(attr.clone());
            });
        if v.len() > 0 {
            return Some(RustFiledAttribute::vec_to_string(v));
        }
        None
    }
}
