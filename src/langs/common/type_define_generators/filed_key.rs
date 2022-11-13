use npc::convertor::NamingPrincipalConvertor;

use super::type_key::TypeKey;

/// FiledKey represent type filed name
/// ```
/// struct Test {
///     // id is FiledKey
///     id: usize
/// }
/// ```
pub(super) struct FiledKey(String);
impl FiledKey {
    pub fn new(key: impl Into<String>) -> Self {
        Self(key.into())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
    pub fn to_type_key(&self) -> TypeKey {
        let npc = NamingPrincipalConvertor::new(&self.0);
        TypeKey::new(npc.to_pascal())
    }
}

#[cfg(test)]
mod test_filed_key {
    use crate::langs::common::type_define_generators::filed_key::FiledKey;

    #[test]
    fn test_to_type_key() {
        let filed_key = FiledKey::new("test");
        assert_eq!(filed_key.to_type_key().value(), "Test");
    }
}
