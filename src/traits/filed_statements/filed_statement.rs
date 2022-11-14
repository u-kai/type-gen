use crate::langs::common::type_define_generators::{
    filed_key::FiledKey, filed_type::FiledType, type_key::TypeKey,
};

pub trait FiledStatement {
    const HEAD_SPACE: &'static str = "    ";
    const FILED_DERIMITA: &'static str = ",";
    fn create_statement(
        &self,
        type_key: &TypeKey,
        filed_key: &FiledKey,
        filed_type: &FiledType,
    ) -> String;
    fn add_head_space(&self, statement: impl Into<String>) -> String {
        format!("{}{}", Self::HEAD_SPACE, statement.into())
    }
}
