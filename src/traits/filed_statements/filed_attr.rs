use crate::langs::common::type_define_generators::{filed_key::FiledKey, type_key::TypeKey};

pub trait FiledAttribute {
    fn get_attr(&self, type_key: &TypeKey, filed_key: &FiledKey) -> Option<String>;
}
