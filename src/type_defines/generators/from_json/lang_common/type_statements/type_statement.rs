use crate::type_defines::statement_parts::type_key::TypeKey;

pub trait TypeStatement {
    const TYPE_STATEMENT: &'static str;
    fn create_statement(&self, type_key: &TypeKey) -> String;
}
