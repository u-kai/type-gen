use std::collections::HashMap;

use crate::{
    traits::optional_checker::OptionalChecker,
    utils::store_fn::{containes_to_kv_vec, push_to_kv_vec},
};

type TypeKey = &'static str;
type OptionFiledKey = &'static str;
type RequireFiledKey = &'static str;
pub struct BaseOptionalChecker {
    default_option_flag: bool,
    optionlas: HashMap<TypeKey, Vec<OptionFiledKey>>,
    requires: HashMap<TypeKey, Vec<OptionFiledKey>>,
}

impl BaseOptionalChecker {
    pub fn new(default_option_flag: bool) -> Self {
        Self {
            default_option_flag,
            optionlas: HashMap::new(),
            requires: HashMap::new(),
        }
    }
    pub fn add_optional(&mut self, type_key: TypeKey, filed_key: OptionFiledKey) {
        push_to_kv_vec(&mut self.optionlas, type_key, filed_key)
    }
    pub fn add_require(&mut self, type_key: TypeKey, filed_key: RequireFiledKey) {
        push_to_kv_vec(&mut self.requires, type_key, filed_key)
    }
}
impl OptionalChecker for BaseOptionalChecker {
    fn is_optional(&self, type_key: &str, filed_key: &str) -> bool {
        if containes_to_kv_vec(&self.requires, &type_key, &filed_key) {
            return false;
        }
        if containes_to_kv_vec(&self.optionlas, &type_key, &filed_key) {
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
        assert!(oc.is_optional("Test", "op"));
        assert!(oc.is_optional("Test", "default"));
        assert!(oc.is_optional("TestData", "default"));
        assert!(!oc.is_optional("Test", "req"));
    }
}
