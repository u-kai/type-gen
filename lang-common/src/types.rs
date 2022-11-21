use std::collections::BTreeMap;

use json::json::Json;

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
    pub fn add_property(&mut self, key: impl Into<PropertyKey>, r#type: impl Into<PropertyType>) {
        self.properties
            .push(Property::new(key.into(), r#type.into()));
    }
}
impl Into<Json> for CompositeType {
    fn into(self) -> Json {
        Json::Object(
            self.properties
                .into_iter()
                .map(|property| property.into())
                .collect::<BTreeMap<String, Json>>(),
        )
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
impl Into<(String, Json)> for Property {
    fn into(self) -> (String, Json) {
        (self.key.0, self.r#type.into())
    }
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
    Optional(Box<PropertyType>),
    Array(Box<PropertyType>),
}
impl Into<Json> for PropertyType {
    fn into(self) -> Json {
        match self {
            Self::Composite(composite) => composite.into(),
            Self::Primitive(primitive) => primitive.into(),
            Self::Optional(property) => {
                let json: PropertyType = *property;
                json.into()
            }
            Self::Array(property) => {
                let json = *property;
                let json: Json = json.into();
                Json::Array(vec![json])
            }
        }
    }
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
impl Into<Json> for PrimitiveType {
    fn into(self) -> Json {
        match self {
            Self::Boolean => Json::Boolean(bool::default()),
            Self::String => Json::String(String::default()),
            Self::Number(num) => match num {
                Number::Float => Json::Number(json::json::Number::from(f64::default())),
                Number::Usize => Json::Number(json::json::Number::from(u64::default())),
                Number::Isize => Json::Number(json::json::Number::from(i64::default())),
            },
        }
    }
}

#[cfg(test)]
mod test_composite_type_into_json {
    use json::json::Json;

    use super::*;

    #[test]
    fn test_simple_case() {
        let tobe = Json::from(r#"{"key":""}"#);
        let mut composition_type = CompositeType::new("Test");
        composition_type.add_property("key", PropertyType::Primitive(PrimitiveType::String));
        let expect: Json = composition_type.into();
        assert_eq!(expect, tobe);
    }
    #[test]
    fn test_nest_case() {
        let tobe = Json::from(r#"{"key":"","obj":{"name":""}}"#);
        let mut child = CompositeType::new("TestoObj");
        child.add_property("name", PropertyType::Primitive(PrimitiveType::String));
        let mut parent = CompositeType::new("Test");
        parent.add_property("key", PropertyType::Primitive(PrimitiveType::String));
        parent.add_property("obj", PropertyType::Composite(child));
        let expect: Json = parent.into();
        assert_eq!(expect, tobe);
    }
    #[test]
    fn test_nest_array_case() {
        let tobe = Json::from(r#"{"key":"","obj":{"name":"","results":[{"name":"","id":0}]}}"#);
        let mut grand_child = CompositeType::new("TestObjResults");
        grand_child.add_property("name", PropertyType::Primitive(PrimitiveType::String));
        grand_child.add_property(
            "id",
            PropertyType::Primitive(PrimitiveType::Number(Number::Usize)),
        );

        let mut child = CompositeType::new("TestObj");
        child.add_property("name", PropertyType::Primitive(PrimitiveType::String));
        child.add_property(
            "results",
            PropertyType::Array(Box::new(PropertyType::Composite(grand_child))),
        );

        let mut parent = CompositeType::new("Test");
        parent.add_property("key", PropertyType::Primitive(PrimitiveType::String));
        parent.add_property("obj", PropertyType::Composite(child));

        let expect: Json = parent.into();
        assert_eq!(expect, tobe);
    }
}
