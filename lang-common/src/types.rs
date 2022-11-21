#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompositeType {
    name: TypeName,
    properties: Vec<Property>,
}

impl CompositeType {
    pub fn new(name: impl Into<TypeName>) -> Self {
        Self {
            name: name.into(),
            properties: vec![],
        }
    }
    pub fn add_property(&mut self) {}
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeName(String);
impl TypeName {
    pub fn new(str: String) -> Self {
        Self(str)
    }
}
impl From<String> for TypeName {
    fn from(str: String) -> Self {
        TypeName::new(str)
    }
}
impl From<&str> for TypeName {
    fn from(str: &str) -> Self {
        TypeName::from(String::from(str))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Property {
    key: PropertyKey,
    r#type: PropertyType,
}

impl Property {
    pub fn new(key: PropertyKey, r#type: PropertyType) -> Self {
        Self { key, r#type }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyKey(String);
impl<I> From<I> for PropertyKey
where
    I: Into<String>,
{
    fn from(source: I) -> Self {
        Self(source.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyType {
    Primitive(PrimitiveType),
    Composite(CompositeType),
}
impl From<CompositeType> for PropertyType {
    fn from(c: CompositeType) -> Self {
        Self::Composite(c)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrimitiveType {
    String,
    Boolean,
    Number(Number),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Number {
    Usize,
    Isize,
    Float,
}

#[cfg(test)]
mod test_composite_type_into_json {
    use json::json::Json;

    use super::CompositeType;

    #[test]
    fn test_simple_case() {
        let tobe = Json::from(r#"{"key":"value"}"#);
        let mut composition_type = CompositeType::new("Test");
        composition_type.add_property()
    }
}
