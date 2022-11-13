use super::filed_key::FiledKey;

/// TypeKey represent type name
/// ```
/// // Test is TypeKey
/// struct Test {
///     id: usize
/// }
/// ```
pub struct TypeKey(String);
impl TypeKey {
    pub fn new(key: impl Into<String>) -> Self {
        Self(key.into())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
    pub fn drain(self) -> String {
        self.0
    }
    pub fn from_parent(parent: &TypeKey, self_filed: &FiledKey) -> Self {
        Self::new(self_filed.to_type_key(parent).drain())
    }
}

#[cfg(test)]
mod test_type_key {
    use crate::langs::common::type_define_generators::filed_key::FiledKey;

    use super::*;
    #[test]
    fn test_from_parent_and_filed_key() {
        let parent = TypeKey::new("Parent");
        let this_filed_key = FiledKey::new("child");
        assert_eq!(
            TypeKey::from_parent(&parent, &this_filed_key).value(),
            "ParentChild"
        );
    }
}
