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
    pub fn add_property(&mut self, property: impl Into<Property>) {
        self.properties.push(property.into())
    }
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyType {
    Primitive(PrimitiveType),
    Composite(CompositeType),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveType(String);
