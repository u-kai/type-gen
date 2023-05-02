use npc::convertor::NamingPrincipalConvertor;

use super::type_name::TypeName;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PropertyKey {
    original: String,
}

impl PropertyKey {
    pub fn to_type_name(&self, parent_type_name: &TypeName) -> TypeName {
        TypeName::new(format!(
            "{}{}",
            parent_type_name.as_str(),
            NamingPrincipalConvertor::new(&self.as_str()).to_pascal()
        ))
    }
    pub fn as_str(&self) -> &str {
        &self.original
    }
    pub fn invalid_lang_str(&self) -> String {
        let corrector = npc::corrector::InvalidCharacterCorrector::default();
        corrector.to_camel(self.as_str())
    }
}
impl<I> From<I> for PropertyKey
where
    I: Into<String>,
{
    fn from(source: I) -> Self {
        let original = source.into();
        Self { original }
    }
}

#[cfg(test)]
mod test {
    use crate::parts::type_name::TypeName;

    use super::PropertyKey;
    #[test]
    fn test_to_type_name() {
        let parent_type_name = TypeName::from("Test");
        let property_key = PropertyKey::from("id");
        assert_eq!(
            property_key.to_type_name(&parent_type_name),
            TypeName::new("TestId".to_string())
        );
    }
    #[test]
    fn test_from_str() {
        let property_key = PropertyKey::from("id");
        assert_eq!(property_key.as_str(), "id");
    }
}
