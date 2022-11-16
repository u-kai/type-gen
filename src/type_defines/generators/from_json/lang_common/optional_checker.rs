use std::collections::HashMap;

use crate::utils::store_fn::{containes_to_kv_vec, push_to_kv_vec};

type TypeKey = &'static str;
type OptionFieldkey = &'static str;
type RequireFieldkey = &'static str;
pub trait OptionalChecker {
    fn is_optional(&self, type_key: &str, field_key: &str) -> bool;
}
pub struct BaseOptionalChecker {
    default_option_flag: bool,
    optionlas: HashMap<TypeKey, Vec<OptionFieldkey>>,
    requires: HashMap<TypeKey, Vec<OptionFieldkey>>,
}

impl BaseOptionalChecker {
    pub fn new(default_option_flag: bool) -> Self {
        Self {
            default_option_flag,
            optionlas: HashMap::new(),
            requires: HashMap::new(),
        }
    }
    pub fn add_optional(&mut self, type_key: TypeKey, field_key: OptionFieldkey) {
        push_to_kv_vec(&mut self.optionlas, type_key, field_key)
    }
    pub fn add_require(&mut self, type_key: TypeKey, field_key: RequireFieldkey) {
        push_to_kv_vec(&mut self.requires, type_key, field_key)
    }
}
impl OptionalChecker for BaseOptionalChecker {
    fn is_optional(&self, type_key: &str, field_key: &str) -> bool {
        if containes_to_kv_vec(&self.requires, &type_key, &field_key) {
            return false;
        }
        if containes_to_kv_vec(&self.optionlas, &type_key, &field_key) {
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
