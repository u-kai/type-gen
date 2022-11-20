use super::statement_parts::{field_key::FieldKey, field_type::FieldType, type_key::TypeKey};

pub struct TypeDefine {
    attr: Option<String>,
    name: TypeKey,
    fields: Vec<TypeDefineField>,
}

struct TypeDefineField {
    name: FieldKey,
    r#type: FieldType,
}
