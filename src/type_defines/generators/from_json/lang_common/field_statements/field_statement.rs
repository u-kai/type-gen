use crate::type_defines::statement_parts::{
    field_key::FieldKey, field_type::FieldType, type_key::TypeKey,
};

pub trait FieldStatement {
    const HEAD_SPACE: &'static str = "    ";
    const FIELD_DERIMITA: &'static str = ",";
    fn create_statement(
        &self,
        type_key: &TypeKey,
        field_key: &FieldKey,
        field_type: &FieldType,
    ) -> String;
    fn add_head_space(&self, statement: impl Into<String>) -> String {
        format!("{}{}", Self::HEAD_SPACE, statement.into())
    }
}
