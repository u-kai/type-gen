use std::collections::HashMap;

use crate::{traits::type_statements::type_attr::TypeAttribution, utils::store_fn::push_to_kv_vec};

#[derive(Debug, Clone)]
pub enum RustTypeAttribute {
    Derive(Vec<String>),
    CfgTest,
    Original(String),
}
impl RustTypeAttribute {
    pub fn vec_to_string(v: Vec<Self>) -> String {
        v.iter()
            .map(|attr| {
                let s: String = attr.into();
                format!("#[{}]\n", s)
            })
            .collect::<Vec<String>>()
            .join("")
    }
    pub fn from_derives(derives: Vec<&str>) -> Self {
        let derives = derives.iter().map(|s| s.to_string()).collect();
        Self::Derive(derives)
    }
}
fn derives_to_statement(v: &[String]) -> String {
    let derives = v.join(",");
    format!("derive({})", derives)
}
impl Into<String> for RustTypeAttribute {
    fn into(self) -> String {
        match self {
            Self::Derive(v) => derives_to_statement(&v),
            Self::CfgTest => "cfg(test)".to_string(),
            Self::Original(s) => s,
        }
    }
}
impl From<&RustTypeAttribute> for String {
    fn from(attr: &RustTypeAttribute) -> Self {
        match attr {
            RustTypeAttribute::Derive(v) => derives_to_statement(v),
            RustTypeAttribute::CfgTest => "cfg(test)".to_string(),
            RustTypeAttribute::Original(s) => s.clone(),
        }
    }
}

pub struct RustTypeAttributeStore {
    all: Option<Vec<RustTypeAttribute>>,
    store: HashMap<String, Vec<RustTypeAttribute>>,
}

impl RustTypeAttributeStore {
    pub fn new() -> Self {
        Self {
            all: None,
            store: HashMap::new(),
        }
    }
    pub fn add_attr(&mut self, key: &str, attr: RustTypeAttribute) {
        push_to_kv_vec(&mut self.store, key.to_string(), attr)
    }
    pub fn set_attr_all(&mut self, attrs: Vec<RustTypeAttribute>) {
        self.all = Some(attrs)
    }
}

impl TypeAttribution for RustTypeAttributeStore {
    fn get_attr(&self, filed_key: &str) -> Option<String> {
        let mut v = Vec::new();
        if let Some(all) = &self.all {
            v.extend(all.clone());
        }
        self.store.get(filed_key).map(|attr| {
            v.extend(attr.clone());
        });
        if v.len() > 0 {
            return Some(RustTypeAttribute::vec_to_string(v));
        }
        None
    }
}

#[cfg(test)]
mod rust_type_attr_test {
    use crate::traits::type_statements::type_attr::TypeAttribution;

    use super::RustTypeAttributeStore;

    #[test]
    fn test_derives() {
        let mut attr = RustTypeAttributeStore::new();
        attr.add_attr(
            "Test",
            super::RustTypeAttribute::Derive(vec!["Serde".to_string(), "Debug".to_string()]),
        );
        assert_eq!(
            attr.get_attr("Test"),
            Some("#[derive(Serde,Debug)]\n".to_string())
        );
        assert_eq!(attr.get_attr("Not"), None);
    }
}
