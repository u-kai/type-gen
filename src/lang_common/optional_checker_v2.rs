use std::collections::HashMap;

use crate::traits::filed_statements::optional_checker::OptionalChecker;

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
}
//pub fn add_optional(&mut self, type_key: &'static str, filed_key: &'static str) {
//self.optionlas.push(key)
//}
//pub fn add_require(&mut self, key: &'static str) {
//self.requires.push(key)
//}
//}

//impl OptionalChecker for BaseOptionalChecker {
//fn is_optional(&self, filed_key: &str) -> bool {
//if self.requires.contains(&filed_key) {
//return false;
//}
//if self.optionlas.contains(&filed_key) {
//return true;
//}
//self.default_option_flag
//}
//}
//impl Default for BaseOptionalChecker {
//fn default() -> Self {
//Self {
//default_option_flag: true,
//optionlas: Vec::new(),
//requires: Vec::new(),
//}
//}
//}

//#[cfg(test)]
//mod test_optional_checker {
//use super::*;
//#[test]
//fn test_optional_checker() {
//let mut oc = BaseOptionalChecker::default();
//oc.add_optional("op");
//oc.add_require("req");
//assert!(oc.is_optional("op"));
//assert!(oc.is_optional("default"));
//assert!(!oc.is_optional("req"));
//}
//}
