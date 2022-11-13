use npc::convertor::NamingPrincipalConvertor;

use super::type_key::TypeKey;

/// FiledKey represent type filed name
/// ```
/// struct Test {
///     // id is FiledKey
///     id: usize
/// }
/// ```
#[derive(Debug, Clone)]
pub struct FiledKey(String);
impl FiledKey {
    pub fn new(key: impl Into<String>) -> Self {
        Self(key.into())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
    pub fn drain(self) -> String {
        self.0
    }
    //pub fn to_filed_type(&self)-> String {

    //}
    pub fn to_type_key(&self, parent: &TypeKey) -> TypeKey {
        let npc = NamingPrincipalConvertor::new(&self.0);
        TypeKey::new(format!("{}{}", parent.value(), npc.to_pascal()))
    }
}

#[cfg(test)]
mod test_filed_key {
    use crate::langs::common::type_define_generators::{filed_key::FiledKey, type_key::TypeKey};

    #[test]
    fn test_to_type_key() {
        let filed_key = FiledKey::new("user_id");
        assert_eq!(
            filed_key.to_type_key(&TypeKey::new("Test")).value(),
            "TestUserId"
        );
    }
}
