use npc::convertor::NamingPrincipalConvertor;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeName(String);
impl TypeName {
    pub fn new(str: impl Into<String>) -> Self {
        Self(str.into())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn valid_lang_str(&self) -> String {
        let corrector = npc::corrector::InvalidCharacterCorrector::default();
        corrector.to_pascal(self.as_str())
    }
}
impl<I> From<I> for TypeName
where
    I: Into<String>,
{
    fn from(str: I) -> Self {
        let str: String = str.into();
        let str = NamingPrincipalConvertor::new(&str).to_pascal();
        TypeName::new(str)
    }
}

impl From<&TypeName> for TypeName {
    fn from(ref_: &TypeName) -> Self {
        ref_.as_str().into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_from_str() {
        let type_name = TypeName::from("test");
        let tobe = TypeName::new("Test".to_string());
        assert_eq!(type_name, tobe);
        assert_eq!(type_name.as_str(), "Test");
    }
}
