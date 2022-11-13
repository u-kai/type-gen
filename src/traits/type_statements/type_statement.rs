use crate::langs::common::type_define_generators::type_key::TypeKey;

pub trait TypeStatement {
    const TYPE_STATEMENT: &'static str;
    fn create_statement(&self, type_key: &TypeKey) -> String;
}
