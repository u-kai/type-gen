use super::field_key::FieldKey;

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
    pub fn from_parent(parent: &TypeKey, self_field: &FieldKey) -> Self {
        Self::new(self_field.to_type_key(parent).drain())
    }
}

#[cfg(test)]
mod test_type_key {

    use crate::type_defines::statement_parts::field_key::FieldKey;

    use super::*;
    #[test]
    fn test_from_parent_and_field_key() {
        let parent = TypeKey::new("Parent");
        let this_field_key = FieldKey::new("child");
        assert_eq!(
            TypeKey::from_parent(&parent, &this_field_key).value(),
            "ParentChild"
        );
    }
}
