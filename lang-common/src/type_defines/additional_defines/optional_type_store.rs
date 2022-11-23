use std::collections::HashMap;

use utils::store_fn::{containes_to_kv_vec, push_to_kv_vec};

use crate::types::{property_key::PropertyKey, type_name::TypeName};

pub struct OptionalTypeStore<'a> {
    default_option_flag: bool,
    optionlas: HashMap<&'a TypeName, Vec<&'a PropertyKey>>,
    requires: HashMap<&'a TypeName, Vec<&'a PropertyKey>>,
}

impl<'a> OptionalTypeStore<'a> {
    pub fn new(default_option_flag: bool) -> Self {
        Self {
            default_option_flag,
            optionlas: HashMap::new(),
            requires: HashMap::new(),
        }
    }
    pub fn add_optional(&mut self, type_name: &'a TypeName, property_key: &'a PropertyKey) {
        push_to_kv_vec(&mut self.optionlas, type_name.into(), property_key.into())
    }
    pub fn add_require(&mut self, type_name: &'a TypeName, property_key: &'a PropertyKey) {
        push_to_kv_vec(&mut self.requires, type_name.into(), property_key.into())
    }
    pub fn is_optional(&self, type_name: &'a TypeName, property_key: &'a PropertyKey) -> bool {
        if containes_to_kv_vec(&self.requires, &type_name, &property_key) {
            return false;
        }
        if containes_to_kv_vec(&self.optionlas, &type_name, &property_key) {
            return true;
        }
        self.default_option_flag
    }
}
impl<'a> Default for OptionalTypeStore<'a> {
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
        let mut oc = OptionalTypeStore::default();
        let type_name: TypeName = "Test".into();
        let property_key1: PropertyKey = "op".into();
        let property_key2: PropertyKey = "req".into();
        oc.add_optional(&type_name, &property_key1);
        oc.add_require(&type_name, &property_key2);
        assert!(oc.is_optional(&"Test".into(), &"op".into()));
        assert!(oc.is_optional(&"Test".into(), &"default".into()));
        assert!(oc.is_optional(&"TestData".into(), &"default".into()));
        assert!(!oc.is_optional(&"Test".into(), &"req".into()));
    }
}
