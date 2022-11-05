use std::collections::HashMap;

use crate::traits::type_statements::type_attr::TypeAttribution;

pub enum RustTypeAttribute {
    Derive(Vec<String>),
    CfgTest,
    Original(String),
}
impl RustTypeAttribute {
    pub fn from_derives(derives: Vec<&str>) -> Self {
        let derives = derives.iter().map(|s| s.to_string()).collect();
        Self::Derive(derives)
    }
}
fn derives_to_statement(v: &[String]) -> String {
    let derives = v.join(",");
    format!("#[derive({})]", derives)
}
impl Into<String> for RustTypeAttribute {
    fn into(self) -> String {
        match self {
            Self::Derive(v) => derives_to_statement(&v),
            Self::CfgTest => "#[cfg(test)]".to_string(),
            Self::Original(s) => s,
        }
    }
}
impl From<&RustTypeAttribute> for String {
    fn from(attr: &RustTypeAttribute) -> Self {
        match attr {
            RustTypeAttribute::Derive(v) => derives_to_statement(v),
            RustTypeAttribute::CfgTest => "#[cfg(test)]".to_string(),
            RustTypeAttribute::Original(s) => s.clone(),
        }
    }
}

pub struct RustTypeAttributeStore {
    store: HashMap<String, RustTypeAttribute>,
}

impl RustTypeAttributeStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
    pub fn set_attr(&mut self, key: &str, attr: RustTypeAttribute) {
        self.store.insert(key.to_string(), attr);
    }
}

impl TypeAttribution for RustTypeAttributeStore {
    fn get_attr(&self, filed_key: &str) -> Option<String> {
        self.store.get(filed_key).map(|attr| {
            let attr: String = attr.into();
            format!("{}\n", attr)
        })
    }
}

#[cfg(test)]
mod rust_type_attr_test {
    use crate::traits::type_statements::type_attr::TypeAttribution;

    use super::RustTypeAttributeStore;

    #[test]
    fn test_derives() {
        let mut attr = RustTypeAttributeStore::new();
        attr.set_attr(
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
