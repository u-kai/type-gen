use crate::type_defines::statement_parts::{field_key::Fieldkey, type_key::TypeKey};

pub trait FieldAttribute {
    fn get_attr(&self, type_key: &TypeKey, field_key: &Fieldkey) -> Option<String>;
}