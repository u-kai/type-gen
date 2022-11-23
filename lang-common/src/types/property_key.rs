use npc::convertor::NamingPrincipalConvertor;

use super::type_name::TypeName;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PropertyKey(String);
impl PropertyKey {
    #[allow(unused)]
    pub(crate) fn to_type_name(&self, parent_type_name: &TypeName) -> TypeName {
        TypeName::new(format!(
            "{}{}",
            parent_type_name.as_str(),
            NamingPrincipalConvertor::new(&self.0).to_pascal()
        ))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl<I> From<I> for PropertyKey
where
    I: Into<String>,
{
    fn from(source: I) -> Self {
        Self(source.into())
    }
}
