use std::collections::{BTreeMap, HashMap};

use json::json::Json;
use npc::convertor::NamingPrincipalConvertor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    name: TypeName,
    kind: TypeKind,
}
impl Type {
    pub fn new(name: impl Into<TypeName>, kind: TypeKind) -> Self {
        Self {
            name: name.into(),
            kind,
        }
    }
    pub fn from_json(name: impl Into<TypeName>, json: Json) -> Self {
        let name = name.into();
        let kind = TypeKind::from_json(json);
        Self { name, kind }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeKind {
    Any,
    Primitive(PrimitiveType),
    Composite(CompositeType),
    Optional(Box<TypeKind>),
    Array(Box<TypeKind>),
}
impl TypeKind {
    fn from_json(json: Json) -> Self {
        match json {
            Json::String(_) => TypeKind::Primitive(PrimitiveType::String),
            Json::Number(num) => TypeKind::Primitive(PrimitiveType::Number(Number::from(num))),
            Json::Boolean(_) => TypeKind::Primitive(PrimitiveType::Boolean),
            Json::Null => TypeKind::Any,
            Json::Array(array) => Self::from_array_json(array),
            Json::Object(obj) => Self::from_obj_json(obj),
        }
    }
    fn from_obj_json(obj: BTreeMap<String, Json>) -> Self {
        let mut properties = HashMap::new();
        for (key, json) in obj {
            let property_key = PropertyKey::from(key);
            let type_kind = Self::from_json(json);
            properties.insert(property_key, type_kind);
        }
        Self::Composite(CompositeType { properties })
    }
    fn from_array_json(array: Vec<Json>) -> Self {
        if array.len() == 0 {
            return Self::Array(Box::new(Self::Any));
        }
        let put_together_json = Json::put_together_array_json(array);
        Self::Array(Box::new(Self::from_json(put_together_json)))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompositeType {
    properties: HashMap<PropertyKey, TypeKind>,
}

impl CompositeType {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeName(String);
impl TypeName {
    pub fn new(str: String) -> Self {
        Self(str)
    }
    fn spawn_child(&self, child: &PropertyKey) -> Self {
        let child = NamingPrincipalConvertor::new(&child.0).to_pascal();
        TypeName(format!("{}{}", self.0, child))
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PropertyKey(String);
impl PropertyKey {
    fn to_type_name(self, parent_type_name: &TypeName) -> TypeName {
        TypeName(format!(
            "{}{}",
            parent_type_name.0,
            NamingPrincipalConvertor::new(&self.0).to_pascal()
        ))
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
impl From<json::json::Number> for Number {
    fn from(num: json::json::Number) -> Self {
        match num {
            json::json::Number::Float64(_) => Self::Float,
            json::json::Number::Isize64(_) => Self::Isize,
            json::json::Number::Usize64(_) => Self::Usize,
        }
    }
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

pub(super) mod hellpers {
    use super::*;
    pub fn make_array_type_easy(content: TypeKind) -> TypeKind {
        TypeKind::Array(Box::new(content))
    }
    pub fn make_composite_type_easy(source: Vec<(&str, TypeKind)>) -> TypeKind {
        let properties = source
            .into_iter()
            .map(|(key, type_kind)| (PropertyKey::from(key), type_kind))
            .collect::<HashMap<_, _>>();
        TypeKind::Composite(CompositeType { properties })
    }
    pub fn type_kind_string() -> TypeKind {
        TypeKind::Primitive(PrimitiveType::String)
    }
    pub fn type_kind_usize() -> TypeKind {
        TypeKind::Primitive(PrimitiveType::Number(Number::Usize))
    }
    pub fn type_kind_isize() -> TypeKind {
        TypeKind::Primitive(PrimitiveType::Number(Number::Isize))
    }
    pub fn type_kind_float() -> TypeKind {
        TypeKind::Primitive(PrimitiveType::Number(Number::Float))
    }
}
#[cfg(test)]
mod test_split {
    use super::*;
    fn test_simple_case() {}
}
#[cfg(test)]
mod test_type_from_json {
    use crate::types::hellpers::{
        make_array_type_easy, make_composite_type_easy, type_kind_string, type_kind_usize,
    };

    use super::*;

    #[test]
    fn test_simple_case() {
        let name = "Test";
        let expect = Type::from_json(name, Json::from(r#"{"key":"value"}"#));
        let mut child = HashMap::new();
        child.insert(
            PropertyKey::from("key"),
            TypeKind::Primitive(PrimitiveType::String),
        );
        let tobe = Type {
            name: name.into(),
            kind: TypeKind::Composite(CompositeType { properties: child }),
        };
        assert_eq!(expect, tobe);
    }
    #[test]
    fn test_obj_case() {
        let name = "Test";
        let json = Json::from(r#"{"name":"kai","obj":{"id":0,"name":"kai"}}"#);
        let expect = Type::from_json(name, json);
        let obj_child = make_composite_type_easy(vec![
            ("name", type_kind_string()),
            ("id", type_kind_usize()),
        ]);
        let tobe = Type {
            name: TypeName::from("Test"),
            kind: make_composite_type_easy(vec![("name", type_kind_string()), ("obj", obj_child)]),
        };
        assert_eq!(expect, tobe);
    }
    #[test]
    fn test_obj_array_case() {
        let name = "Test";
        let json = r#"
        {
            "id":0,
            "name":"kai",
            "data": [
                {
                    "id":0,
                    "results":[
                        {
                            "id":10000,
                            "data":"data"
                        }
                    ]
                },
                {
                    "age":26
                },
                {
                    "name":"kai",
                    "results":[
                        {
                            "score":1000
                        }
                    ]
                }
            ]
        }
        "#;
        let expect = Type::from_json(name, Json::from(json));
        let child = make_composite_type_easy(vec![
            ("id", type_kind_usize()),
            ("name", type_kind_string()),
            (
                "data",
                make_array_type_easy(make_composite_type_easy(vec![
                    ("id", type_kind_usize()),
                    ("age", type_kind_usize()),
                    ("name", type_kind_string()),
                    (
                        "results",
                        make_array_type_easy(make_composite_type_easy(vec![
                            ("data", type_kind_string()),
                            ("id", type_kind_usize()),
                            ("score", type_kind_usize()),
                        ])),
                    ),
                ])),
            ),
        ]);
        let tobe = Type {
            name: name.into(),
            kind: child,
        };
        assert_eq!(expect, tobe);
    }
    #[test]
    fn test_primitive_array_case() {
        let name = "Test";
        let expect = Type::from_json(name, Json::from(r#"{"key":["value"]}"#));
        let mut child = HashMap::new();
        child.insert(
            PropertyKey::from("key"),
            TypeKind::Array(Box::new(TypeKind::Primitive(PrimitiveType::String))),
        );
        let tobe = Type {
            name: name.into(),
            kind: TypeKind::Composite(CompositeType { properties: child }),
        };
        assert_eq!(expect, tobe);
    }
}
