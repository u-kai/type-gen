use npc::convertor::NamingPrincipalConvertor;

use crate::langs::common::{naming_principal::NamingPrincipal, utils::replace_cannot_use_char};

use super::type_key::TypeKey;

/// FiledKey represent type filed name
/// ```
/// struct Test {
///     // id is FiledKey
///     id: usize
/// }
/// ```
#[derive(Debug, Clone)]
pub struct FiledKey {
    original: String,
}
impl FiledKey {
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
mod test_filed_key {
    use crate::langs::common::{
        naming_principal::NamingPrincipal,
        type_define_generators::{filed_key::FiledKey, type_key::TypeKey},
    };
    #[test]
    fn test_rename() {
        let filed_key = FiledKey::new("user:id");
        assert_eq!(filed_key.original(), "user:id");
        assert_eq!(filed_key.rename(NamingPrincipal::Camel), "userid");
    }

    #[test]
    fn test_to_type_key() {
        let filed_key = FiledKey::new("user_id");
        assert_eq!(
            filed_key.to_type_key(&TypeKey::new("Test")).value(),
            "TestUserId"
        );
    }
}
