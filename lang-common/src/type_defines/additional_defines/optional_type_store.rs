use std::collections::HashMap;

use utils::store_fn::{containes_to_kv_vec, push_to_kv_vec};

use crate::types::{property_key::PropertyKey, type_name::TypeName};

#[derive(Debug, Clone)]
pub struct OptionalTypeStore {
    all_option_flag: Option<bool>,
    default_option_flag: bool,
    optionlas: HashMap<TypeName, Vec<PropertyKey>>,
    requires: HashMap<TypeName, Vec<PropertyKey>>,
}

impl OptionalTypeStore {
    pub fn new(default_option_flag: bool) -> Self {
        Self {
            all_option_flag: None,
            default_option_flag,
            optionlas: HashMap::new(),
            requires: HashMap::new(),
        }
    }
    pub fn set_all_is_optional(&mut self, bool: bool) {
        self.all_option_flag = Some(bool)
    }
    pub fn add_optional(
        &mut self,
        type_name: impl Into<TypeName>,
        property_key: impl Into<PropertyKey>,
    ) {
        push_to_kv_vec(&mut self.optionlas, type_name.into(), property_key.into())
    }
    pub fn add_require(
        &mut self,
        type_name: impl Into<TypeName>,
        property_key: impl Into<PropertyKey>,
    ) {
        push_to_kv_vec(&mut self.requires, type_name.into(), property_key.into())
    }
    pub fn is_optional(&self, type_name: &TypeName, property_key: &PropertyKey) -> bool {
        if self.all_option_flag.is_some() {
            return self.all_option_flag.unwrap();
        }
        if containes_to_kv_vec(&self.requires, &type_name, &property_key) {
            return false;
        }
        if containes_to_kv_vec(&self.optionlas, &type_name, &property_key) {
            return true;
        }
        self.default_option_flag
    }
}
impl Default for OptionalTypeStore {
    fn default() -> Self {
        Self {
            all_option_flag: None,
            default_option_flag: true,
            optionlas: HashMap::new(),
            requires: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod test_optional_checker {

    use super::*;
    #[test]
    fn test_optional_checker() {
        let mut oc = OptionalTypeStore::default();
        let type_name = "Test";
        let property_key1 = "op";
        let property_key2 = "req";
        oc.add_optional(type_name, property_key1);
        oc.add_require(type_name, property_key2);
        assert!(oc.is_optional(&"Test".into(), &"op".into()));
        assert!(oc.is_optional(&"Test".into(), &"default".into()));
        assert!(oc.is_optional(&"TestData".into(), &"default".into()));
        assert!(!oc.is_optional(&"Test".into(), &"req".into()));
        oc.set_all_is_optional(true);
        assert!(oc.is_optional(&"Test".into(), &"op".into()));
        assert!(oc.is_optional(&"Test".into(), &"default".into()));
        assert!(oc.is_optional(&"TestData".into(), &"default".into()));
        assert!(oc.is_optional(&"Test".into(), &"req".into()));
    }
}
