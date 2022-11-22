use std::collections::{BTreeMap, HashMap};

use json::json::Json;
use npc::convertor::NamingPrincipalConvertor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    name: TypeName,
    kind: TypeKind,
}
impl Type {
    fn new(name: impl Into<TypeName>, kind: TypeKind) -> Self {
        Self {
            name: name.into(),
            kind,
        }
    }
    fn from_json(name: impl Into<TypeName>, json: Json) -> Self {
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
            return TypeKind::Array(Box::new(TypeKind::Any));
        }

        //let collect_obj = Json::collect_obj_from_json_array(array);

        TypeKind::Any
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompositeType {
    properties: HashMap<PropertyKey, TypeKind>,
}

impl CompositeType {
    pub fn from_json(name: impl Into<TypeName>, json: Json) -> Self {
        let name = name.into();
        //let properties = Property::from_json(&name, json);
        //Self { name, properties }
        Self {
            properties: HashMap::new(),
        }
    }
    fn from_obj_json(
        parent_name: &TypeName,
        this_property_key: PropertyKey,
        obj: BTreeMap<String, Json>,
    ) -> Self {
        //let this_name = this_property_key.to_type_name(parent_name);
        Self {
            properties: HashMap::new(),
        }
        //Self {
        //properties: Property::from_obj_json(&this_name, obj),
        //name: this_name,
        //}
    }
    fn add_property(&mut self, key: impl Into<PropertyKey>, r#type: impl Into<Type>) {
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
//#[derive(Debug, Clone, PartialEq, Eq)]
//pub struct Property {
//key: PropertyKey,
//r#type: TypeKind,
//}
//impl Property {
//fn new(key: impl Into<PropertyKey>, r#type: TypeKind) -> Self {
//Self {
//key: key.into(),
//r#type,
//}
//}
//fn from_json(parent_name: &TypeName, json: Json) -> Vec<Self> {
//match json {
//Json::Array(array) => Self::from_array_json(array),
//Json::Object(obj) => Self::from_obj_json(parent_name, obj),
//_ => panic!("this case is not property {:#?}", json),
//}
//}
//fn from_obj_json(parent_name: &TypeName, obj: BTreeMap<String, Json>) -> Vec<Self> {
//let mut result = Vec::new();
//obj.into_iter().for_each(|(key, json)| {
//let child_property_key = PropertyKey(key);
//let child_type_name = parent_name.spawn_child(&child_property_key);
//result.push(Self::new(
//child_property_key,
//TypeKind::from_json(&child_type_name, json),
//))
//});
//result
//}
//fn from_array_json(array: Vec<Json>) -> Vec<Self> {
//if array.len() == 0 {
////return Type::Array(Box::new(Type::Any));
//}
//vec![]
//}
//}
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

#[cfg(test)]
mod test_type_from_json {
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
        let mut properties = HashMap::new();
        properties.insert(
            PropertyKey::from("name"),
            TypeKind::Primitive(PrimitiveType::String),
        );
        let mut obj_child = HashMap::new();
        obj_child.insert(
            PropertyKey::from("name"),
            TypeKind::Primitive(PrimitiveType::String),
        );
        obj_child.insert(
            PropertyKey::from("id"),
            TypeKind::Primitive(PrimitiveType::Number(Number::Usize)),
        );
        properties.insert(
            PropertyKey::from("obj"),
            TypeKind::Composite(CompositeType {
                properties: obj_child,
            }),
        );
        let tobe = Type {
            name: TypeName::from("Test"),
            kind: TypeKind::Composite(CompositeType { properties }),
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
        //assert_eq!(expect, tobe);
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
        //tobe.add_property("key", Type::Primitive(PrimitiveType::String));
        //assert_eq!(expect, tobe);
    }
    //#[test]
    //fn test_nest_case() {
    //let tobe = Json::from(r#"{"key":"","obj":{"name":""}}"#);
    //let mut child = CompositeType::new("TestoObj");
    //child.add_property("name", Type::Primitive(PrimitiveType::String));
    //let mut parent = CompositeType::new("Test");
    //parent.add_property("key", Type::Primitive(PrimitiveType::String));
    //parent.add_property("obj", Type::Composite(child));
    ////let expect: Json = parent.into();
    ////assert_eq!(expect, tobe);
    //}
    //#[test]
    //fn test_array_case() {
    //let tobe = Json::from(r#"{"key":"","obj":{"name":"","results":[{"name":"","id":0}]}}"#);
    //let mut grand_child = CompositeType::new("TestObjResults");
    //grand_child.add_property("name", Type::Primitive(PrimitiveType::String));
    //grand_child.add_property(
    //"id",
    //Type::Primitive(PrimitiveType::Number(Number::Usize)),
    //);

    //let mut child = CompositeType::new("TestObj");
    //child.add_property("name", Type::Primitive(PrimitiveType::String));
    //child.add_property(
    //"results",
    //Type::Array(Box::new(Type::Composite(grand_child))),
    //);

    //let mut parent = CompositeType::new("Test");
    //parent.add_property("key", Type::Primitive(PrimitiveType::String));
    //parent.add_property("obj", Type::Composite(child));

    ////let expect: Json = parent.into();
    ////assert_eq!(expect, tobe);
    //}
    //#[test]
    //fn test_nest_array_case() {
    //let json = r#"{
    //"key":"",
    //"obj":{
    //"name":"",
    //"results":[
    //{
    //"name":"",
    //"id":0,
    //"datas":[
    //{
    //"id":0
    //}
    //]
    //},{
    //"name":"",
    //"id":0,
    //"datas":[
    //{
    //"id":0
    //}
    //]
    //}
    //]
    //}
    //}"#;
    //let tobe = Json::from(json);
    //let mut grand_child = CompositeType::new("TestObjResults");
    //grand_child.add_property("name", Type::Primitive(PrimitiveType::String));
    //grand_child.add_property(
    //"id",
    //Type::Primitive(PrimitiveType::Number(Number::Usize)),
    //);

    //let mut child = CompositeType::new("TestObj");
    //child.add_property("name", Type::Primitive(PrimitiveType::String));
    //child.add_property(
    //"results",
    //Type::Array(Box::new(Type::Composite(grand_child))),
    //);

    //let mut parent = CompositeType::new("Test");
    //parent.add_property("key", Type::Primitive(PrimitiveType::String));
    //parent.add_property("obj", Type::Composite(child));

    ////let expect: Json = parent.into();
    ////assert_eq!(expect, tobe);
    //}
}
