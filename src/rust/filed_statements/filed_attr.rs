use std::collections::HashMap;

use crate::{
    traits::filed_statements::filed_attr::FiledAttribute, utils::store_fn::push_to_kv_vec,
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

pub struct RustFiledAttributeStore {
    all: Option<Vec<RustFiledAttribute>>,
    store: HashMap<String, Vec<RustFiledAttribute>>,
}

impl RustFiledAttributeStore {
    pub fn new() -> Self {
        Self {
            all: None,
            store: HashMap::new(),
        }
    }
    pub fn set_attr_all(&mut self, attrs: Vec<RustFiledAttribute>) {
        self.all = Some(attrs)
    }
    pub fn add_attr(&mut self, key: &str, attr: RustFiledAttribute) {
        push_to_kv_vec(&mut self.store, key.to_string(), attr)
    }
}

impl FiledAttribute for RustFiledAttributeStore {
    fn get_attr(&self, filed_key: &str) -> Option<String> {
        let mut v = Vec::new();
        if let Some(all) = &self.all {
            v.extend(all.clone());
        }
        self.store.get(filed_key).map(|attr| {
            v.extend(attr.clone());
        });
        if v.len() > 0 {
            return Some(RustFiledAttribute::vec_to_string(v));
        }
        None
    }
}
