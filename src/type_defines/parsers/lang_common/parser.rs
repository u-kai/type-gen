use std::collections::BTreeMap;

use crate::langs::rust::rust_visibility::RustVisibility;

//fn parse_type_statement<'a>(source: &'a str, type_key: &str) -> Option<&str> {}
type FieldKey = String;
type fieldValue = String;
pub struct ObjectStatement {
    name: String,
    visibility: RustVisibility,
    field: BTreeMap<FieldKey, fieldValue>,
}
//impl From<&str> for ObjectStatement {
//fn from(source: &str) -> Self {}
//}

impl ObjectStatement {
    pub fn new(name: impl Into<String>, visibility: RustVisibility) -> Self {
        Self {
            name: name.into(),
            visibility,
            field: BTreeMap::new(),
        }
    }
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.field.insert(key.into(), value.into());
    }
}
#[cfg(test)]
mod test_parser {
    use super::*;
    use crate::langs::rust::rust_visibility::RustVisibility;
    #[test]
    fn test_rust_parser() {
        let statement = r#"
        struct Test {
            id:usize,
            name:String,
        };
        "#;

        let mut rust_statement = ObjectStatement::new("Test", RustVisibility::Private);
        rust_statement.insert("id", "usize");
        rust_statement.insert("name", "String");
        //assert_eq!(ObjectStatement::from(statement), rust_statement);
    }
}
