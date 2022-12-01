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
