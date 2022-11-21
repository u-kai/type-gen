use npc::convertor::NamingPrincipalConvertor;

use crate::type_defines::generators::from_json::lang_common::naming_principal::NamingPrincipal;

use super::type_key::TypeKey;

pub fn replace_cannot_use_char(str: &str) -> String {
    str.replace(
        |c| match c {
            ':' | ';' | '#' | '$' | '%' | '&' | '~' | '=' | '|' | '\"' | '\'' | '{' | '}' | '?'
            | '!' | '<' | '>' | '[' | ']' | '*' | '^' => true,
            _ => false,
        },
        "",
    )
}
/// FieldKey represent type field name
/// ```
/// struct Test {
///     // id is FieldKey
///     id: usize
/// }
/// ```
#[derive(Debug, Clone)]
pub struct FieldKey {
    original: String,
}
impl FieldKey {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            original: key.into(),
        }
    }
    pub fn original(&self) -> &str {
        &self.original
    }
    pub fn drain_original(self) -> String {
        self.original
    }
    pub fn to_type_key(&self, parent: &TypeKey) -> TypeKey {
        let renamed = &self.rename(NamingPrincipal::Pascal);
        TypeKey::new(format!("{}{}", parent.value(), renamed))
    }
    pub fn rename(&self, nameing_principal: NamingPrincipal) -> String {
        let remove_cannot_use = replace_cannot_use_char(&self.original);
        match nameing_principal {
            NamingPrincipal::Camel => NamingPrincipalConvertor::new(&remove_cannot_use).to_camel(),
            NamingPrincipal::Snake => NamingPrincipalConvertor::new(&remove_cannot_use).to_snake(),
            NamingPrincipal::Pascal => {
                NamingPrincipalConvertor::new(&remove_cannot_use).to_pascal()
            }
            NamingPrincipal::Chain => NamingPrincipalConvertor::new(&remove_cannot_use).to_chain(),
            NamingPrincipal::Constant => {
                NamingPrincipalConvertor::new(&remove_cannot_use).to_constant()
            }
        }
    }
}

#[cfg(test)]
mod test_field_key {
    use crate::type_defines::{
        generators::from_json::lang_common::naming_principal::NamingPrincipal,
        statement_parts::{field_key::FieldKey, type_key::TypeKey},
    };

    #[test]
    fn test_rename() {
        let field_key = FieldKey::new("user:id");
        assert_eq!(field_key.original(), "user:id");
        assert_eq!(field_key.rename(NamingPrincipal::Camel), "userid");
    }

    #[test]
    fn test_to_type_key() {
        let field_key = FieldKey::new("user_id");
        assert_eq!(
            field_key.to_type_key(&TypeKey::new("Test")).value(),
            "TestUserId"
        );
    }
}
