#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeName(String);
impl TypeName {
    pub fn new(str: String) -> Self {
        Self(str)
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl<I> From<I> for TypeName
where
    I: Into<String>,
{
    fn from(str: I) -> Self {
        let str: String = str.into();
        TypeName::new(str)
    }
}

impl From<&TypeName> for TypeName {
    fn from(ref_: &TypeName) -> Self {
        ref_.as_str().into()
    }
}
