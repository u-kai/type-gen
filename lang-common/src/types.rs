use std::collections::BTreeMap;

use json::json::Json;
use npc::convertor::NamingPrincipalConvertor;

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
    //pub fn from_json(name: impl Into<TypeName>, json: Json) -> Self {
    //let properties = Property::from(json);
    //}
    fn add_property(&mut self, key: impl Into<PropertyKey>, r#type: impl Into<PropertyType>) {
        //self.properties
        //.push(Property{key:key.into(),r#type :r#type.into()});
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeName(String);
impl TypeName {
    pub fn new(str: String) -> Self {
        Self(str)
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Property {
    key: PropertyKey,
    r#type: PropertyType,
}
impl Property {
    fn new(key: impl Into<PropertyKey>, r#type: PropertyType) -> Self {
        Self {
            key: key.into(),
            r#type,
        }
    }
    fn from_json(json: Json) -> Vec<Self> {
        match json {
            Json::Array(array) => Self::from_array_json(array),
            Json::Object(obj) => Self::from_obj_json(obj),
            _ => panic!("this case is not property {:#?}", json),
        }
    }
    fn from_obj_json(obj: BTreeMap<String, Json>) -> Vec<Self> {
        let mut result = Vec::new();
        obj.into_iter()
            .for_each(|(key, json)| result.push(Self::new(key, PropertyType::from(json))));
        result
    }
    fn from_array_json(array: Vec<Json>) -> Vec<Self> {
        if array.len() == 0 {
            //return PropertyType::Array(Box::new(PropertyType::Any));
        }
        vec![]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
pub enum PropertyType {
    Any,
    Primitive(PrimitiveType),
    Composite(CompositeType),
    Optional(Box<PropertyType>),
    Array(Box<PropertyType>),
}
impl PropertyType {
    fn from_obj_json(obj: BTreeMap<String, Json>) -> Self {
        PropertyType::Any
    }
    fn from_array_json(array: Vec<Json>) -> Self {
        if array.len() == 0 {
            return PropertyType::Array(Box::new(PropertyType::Any));
        }
        PropertyType::Any
    }
}

impl From<Json> for PropertyType {
    fn from(json: Json) -> Self {
        match json {
            Json::String(_) => PropertyType::Primitive(PrimitiveType::String),
            Json::Number(num) => PropertyType::Primitive(PrimitiveType::Number(Number::from(num))),
            Json::Boolean(_) => PropertyType::Primitive(PrimitiveType::Boolean),
            Json::Null => PropertyType::Any,
            Json::Array(array) => Self::from_array_json(array),
            Json::Object(obj) => Self::from_obj_json(obj),
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

#[cfg(test)]
mod test_properties_from_json {
    use super::*;

    #[test]
    fn test_simple_case() {
        let expect = Property::from_json(Json::from(r#"{"key":"value"}"#));
        let tobe = vec![Property::new(
            "key",
            PropertyType::Primitive(PrimitiveType::String),
        )];
        assert_eq!(expect, tobe);
    }
}
#[cfg(test)]
mod test_composite_type_into_json {
    use json::json::Json;

    use super::*;

    #[test]
    fn test_simple_case() {
        let name = "Test";
        //let expect = CompositeType::from_json(name, Json::from(r#"{"key":"value"}"#));
        //let mut tobe = CompositeType::new(name);
        //tobe.add_property("key", PropertyType::Primitive(PrimitiveType::String));
        //assert_eq!(expect, tobe);
    }
    #[test]
    fn test_nest_case() {
        let tobe = Json::from(r#"{"key":"","obj":{"name":""}}"#);
        let mut child = CompositeType::new("TestoObj");
        child.add_property("name", PropertyType::Primitive(PrimitiveType::String));
        let mut parent = CompositeType::new("Test");
        parent.add_property("key", PropertyType::Primitive(PrimitiveType::String));
        parent.add_property("obj", PropertyType::Composite(child));
        //let expect: Json = parent.into();
        //assert_eq!(expect, tobe);
    }
    #[test]
    fn test_array_case() {
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

        //let expect: Json = parent.into();
        //assert_eq!(expect, tobe);
    }
    #[test]
    fn test_nest_array_case() {
        let json = r#"{
            "key":"",
            "obj":{
                "name":"",
                "results":[
                    {
                        "name":"",
                        "id":0,
                        "datas":[
                            {
                                "id":0
                            }
                        ]
                    },{
                        "name":"",
                        "id":0,
                        "datas":[
                            {
                                "id":0
                            }
                        ]
                    }
                ]
            }
        }"#;
        let tobe = Json::from(json);
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

        //let expect: Json = parent.into();
        //assert_eq!(expect, tobe);
    }
}
