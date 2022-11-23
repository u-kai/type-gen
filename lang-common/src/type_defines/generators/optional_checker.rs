use std::collections::HashMap;

use utils::store_fn::{containes_to_kv_vec, push_to_kv_vec};

use crate::types::structures::{PropertyKey, TypeName};

pub trait OptionalChecker {
    fn is_optional(&self, type_name: &TypeName, property_key: &PropertyKey) -> bool;
}
pub struct BaseOptionalChecker {
    default_option_flag: bool,
    optionlas: HashMap<TypeName, Vec<PropertyKey>>,
    requires: HashMap<TypeName, Vec<PropertyKey>>,
}

impl BaseOptionalChecker {
    pub fn new(default_option_flag: bool) -> Self {
        Self {
            default_option_flag,
            optionlas: HashMap::new(),
            requires: HashMap::new(),
        }
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
}
impl OptionalChecker for BaseOptionalChecker {
    fn is_optional(&self, type_name: &TypeName, property_key: &PropertyKey) -> bool {
        if containes_to_kv_vec(&self.requires, type_name, property_key) {
            return false;
        }
        if containes_to_kv_vec(&self.optionlas, type_name, property_key) {
            return true;
        }
        self.default_option_flag
    }
}
impl Default for BaseOptionalChecker {
    fn default() -> Self {
        Self {
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
        let mut oc = BaseOptionalChecker::default();
        oc.add_optional("Test", "op");
        oc.add_require("Test", "req");
        assert!(oc.is_optional(&"Test".into(), &"op".into()));
        assert!(oc.is_optional(&"Test".into(), &"default".into()));
        assert!(oc.is_optional(&"TestData".into(), &"default".into()));
        assert!(!oc.is_optional(&"Test".into(), &"req".into()));
    }
}
