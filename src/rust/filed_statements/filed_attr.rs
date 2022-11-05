use std::collections::HashMap;

use crate::traits::filed_statements::filed_attr::FiledAttribute;

pub enum RustFiledAttribute {
    CfgTest,
    Original(String),
}
impl RustFiledAttribute {
    pub fn vec_to_string(v: &Vec<Self>) -> String {
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
    store: HashMap<String, Vec<RustFiledAttribute>>,
}

impl RustFiledAttributeStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
    pub fn set_attr(&mut self, key: &str, attr: RustFiledAttribute) {
        if let Some(v) = self.store.get_mut(key) {
            v.push(attr);
            return;
        }
        self.store.insert(key.to_string(), vec![attr]);
    }
}

impl FiledAttribute for RustFiledAttributeStore {
    fn get_attr(&self, filed_key: &str) -> Option<String> {
        self.store
            .get(filed_key)
            .map(|attr| RustFiledAttribute::vec_to_string(attr))
    }
}
