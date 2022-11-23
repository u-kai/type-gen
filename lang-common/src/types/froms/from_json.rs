use std::collections::{BTreeMap, HashMap};

use json::json::Json;

use crate::types::structure::{
    CompositeType, Number, PrimitiveType, PropertyKey, TypeKind, TypeName, TypeStructure,
};

impl TypeStructure {
    pub fn from_json(root: impl Into<TypeName>, json: Json) -> Self {
        let name = root.into();
        let kind = TypeKind::from_json(&name, json);
        Self::new(name, kind)
    }
    pub fn from_json_to_nest_type(name: impl Into<TypeName>, json: Json) -> Self {
        let name = name.into();
        let kind = TypeKind::from_json_to_nest_type_kind(json);
        Self::new(name, kind)
    }
}
impl TypeKind {
    pub fn from_json(root_name: &TypeName, json: Json) -> Self {
        println!("from_root_json = {:?}", root_name);
        match json {
            Json::String(_) => TypeKind::Primitive(PrimitiveType::String),
            Json::Number(num) => TypeKind::Primitive(PrimitiveType::Number(Number::from(num))),
            Json::Boolean(_) => TypeKind::Primitive(PrimitiveType::Boolean),
            Json::Null => TypeKind::Any,
            Json::Array(array) => Self::from_root_array_json(root_name.clone(), array),
            Json::Object(obj) => {
                let mut properties = HashMap::new();
                for (key, json) in obj {
                    let property_key = PropertyKey::from(key);
                    let child_type_name = property_key.to_type_name(root_name);
                    let type_kind = match json {
                        Json::Object(obj) => {
                            let kind = Self::from_obj_json(&child_type_name, &property_key, obj);
                            let child_type = TypeStructure::new(child_type_name, kind);
                            Self::ChildType(Box::new(child_type))
                        }
                        Json::Array(array) => {
                            Self::from_array_json(root_name, &property_key, array)
                        }
                        Json::String(_) => TypeKind::Primitive(PrimitiveType::String),
                        Json::Number(num) => {
                            TypeKind::Primitive(PrimitiveType::Number(Number::from(num)))
                        }
                        Json::Boolean(_) => TypeKind::Primitive(PrimitiveType::Boolean),
                        Json::Null => TypeKind::Any,
                    };
                    properties.insert(property_key, type_kind);
                }
                Self::Composite(CompositeType::new(properties))
            }
        }
    }
    fn obj_json_to_composite_type(parent_name: &TypeName, obj: BTreeMap<String, Json>) -> Self {
        let mut properties = HashMap::new();
        for (key, json) in obj {
            let property_key = PropertyKey::from(key);
            let child_type_name = property_key.to_type_name(parent_name);
            let type_kind = match json {
                Json::Object(obj) => {
                    let kind = Self::from_obj_json(&child_type_name, &property_key, obj);
                    let child_type = TypeStructure::new(child_type_name, kind);
                    Self::ChildType(Box::new(child_type))
                }
                Json::Array(array) => Self::from_array_json(parent_name, &property_key, array),
                Json::String(_) => TypeKind::Primitive(PrimitiveType::String),
                Json::Number(num) => TypeKind::Primitive(PrimitiveType::Number(Number::from(num))),
                Json::Boolean(_) => TypeKind::Primitive(PrimitiveType::Boolean),
                Json::Null => TypeKind::Any,
            };
            properties.insert(property_key, type_kind);
        }
        Self::Composite(CompositeType::new(properties))
    }
    fn from_obj_json(
        parent_name: &TypeName,
        property_key: &PropertyKey,
        obj: BTreeMap<String, Json>,
    ) -> Self {
        let mut properties = HashMap::new();
        let obj_name = property_key.to_type_name(parent_name);
        for (key, json) in obj {
            let property_key = PropertyKey::from(key);
            let child_type_name = property_key.to_type_name(&obj_name);
            let type_kind = match json {
                Json::Object(obj) => {
                    //             let kind = Self::from_obj_json(&obj_name, &property_key, obj);
                    let kind = Self::obj_json_to_composite_type(&obj_name, obj);
                    println!();
                    println!("obj_name {:?}", obj_name);
                    println!("property_key {:?}", property_key);
                    println!("kind {:#?}", kind);
                    println!();
                    let child_type = TypeStructure::new(child_type_name, kind);
                    Self::ChildType(Box::new(child_type))
                }
                Json::Array(array) => Self::from_array_json(&child_type_name, &property_key, array),
                Json::String(_) => TypeKind::Primitive(PrimitiveType::String),
                Json::Number(num) => TypeKind::Primitive(PrimitiveType::Number(Number::from(num))),
                Json::Boolean(_) => TypeKind::Primitive(PrimitiveType::Boolean),
                Json::Null => TypeKind::Any,
            };
            properties.insert(property_key, type_kind);
        }
        let child_type_kind = Self::Composite(CompositeType::new(properties));
        Self::ChildType(Box::new(TypeStructure::new(obj_name, child_type_kind)))
    }
    fn from_array_json(
        parent_name: &TypeName,
        property_key: &PropertyKey,
        array: Vec<Json>,
    ) -> Self {
        if array.len() == 0 {
            return Self::Array(Box::new(Self::Any));
        }
        let put_together_json = Json::put_together_array_json(array);
        let type_kind = match put_together_json {
            Json::Object(obj) => Self::from_obj_json(parent_name, property_key, obj),
            _ => Self::from_json(parent_name, put_together_json),
        };
        Self::Array(Box::new(type_kind))
    }
    fn from_root_array_json(parent_name: TypeName, array: Vec<Json>) -> Self {
        if array.len() == 0 {
            return Self::Array(Box::new(Self::Any));
        }
        let put_together_json = Json::put_together_array_json(array);
        Self::Array(Box::new(Self::ChildType(Box::new(
            TypeStructure::from_json(parent_name, put_together_json),
        ))))
    }
    fn from_json_to_nest_type_kind(json: Json) -> Self {
        match json {
            Json::String(_) => TypeKind::Primitive(PrimitiveType::String),
            Json::Number(num) => TypeKind::Primitive(PrimitiveType::Number(Number::from(num))),
            Json::Boolean(_) => TypeKind::Primitive(PrimitiveType::Boolean),
            Json::Null => TypeKind::Any,
            Json::Array(array) => Self::from_array_json_to_nest_type_kind(array),
            Json::Object(obj) => Self::from_obj_json_to_nest_type_kind(obj),
        }
    }
    fn from_obj_json_to_nest_type_kind(obj: BTreeMap<String, Json>) -> Self {
        let mut properties = HashMap::new();
        for (key, json) in obj {
            let property_key = PropertyKey::from(key);
            let type_kind = Self::from_json_to_nest_type_kind(json);
            properties.insert(property_key, type_kind);
        }
        Self::Composite(CompositeType::new(properties))
    }
    fn from_array_json_to_nest_type_kind(array: Vec<Json>) -> Self {
        if array.len() == 0 {
            return Self::Array(Box::new(Self::Any));
        }
        let put_together_json = Json::put_together_array_json(array);
        Self::Array(Box::new(Self::from_json_to_nest_type_kind(
            put_together_json,
        )))
    }
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
    use crate::types::structure::fakes::*;
    #[test]
    fn test_complex_case() {
        let json = r#"{
            "root":[
                    {
                        "id":0,
                        "name":"kai",
                        "data": {
                            "age":26,
                            "details":[
                                {
                                    "likes":["hamabe","imada"],
                                    "hobby":"rust"
                                },
                                {
                                    "userId":10000
                                }
                            ]
                        }
                    },
                    {
                        "id":0,
                        "age":25
                    },
                    {
                        "id":0,
                        "age":35,
                        "data": {
                            "age":26,
                            "from":"kanagawa",
                            "details":[
                                {
                                    "frends":["hamabe","imada"]
                                },
                                {
                                    "frendId":10000
                                }
                            ]
                        },
                        "details":[
                            {
                                "likes":["hamabe","imada"],
                                "hobby":"rust"
                            },
                            {
                                "userId":10000
                            }
                        ]
                    }
            ]
        }
        "#;
        let name = "Test";
        let data = make_type_easy(
            "TestRootData",
            make_array_type_easy(make_composite_type_easy(vec![
                ("age", type_kind_usize()),
                ("from", type_kind_string()),
                (
                    "details",
                    make_child_type(make_type_easy(
                        "TestRootDataDetails",
                        make_array_type_easy(make_composite_type_easy(vec![
                            ("likes", make_array_type_easy(type_kind_string())),
                            ("hobby", type_kind_string()),
                            ("userId", type_kind_usize()),
                            ("frendId", type_kind_usize()),
                            ("frends", make_array_type_easy(type_kind_string())),
                        ])),
                    )),
                ),
            ])),
        );
        let details = make_type_easy(
            "TestRootDetails",
            make_array_type_easy(make_composite_type_easy(vec![
                ("likes", make_array_type_easy(type_kind_string())),
                ("hobby", type_kind_string()),
                ("userId", type_kind_usize()),
            ])),
        );
        let kind = make_composite_type_easy(vec![(
            "root",
            make_array_type_easy(make_child_type(make_type_easy(
                "TestRoot",
                make_array_type_easy(make_composite_type_easy(vec![
                    ("id", type_kind_usize()),
                    ("age", type_kind_usize()),
                    ("name", type_kind_string()),
                    ("data", make_child_type(data)),
                    ("details", make_child_type(details)),
                ])),
            ))),
        )]);
        let tobe = TypeStructure::new(name, kind);
        let expect = TypeStructure::from_json(name, Json::from(json));
        //println!("tobe {:#?}", tobe);
        //println!("expect {:#?}", expect);
        //assert_eq!(expect, tobe);
    }
}
#[cfg(test)]
mod test_type_from_json_to_nest_type {

    use crate::types::structure::fakes::*;

    use super::*;
    #[test]
    fn test_complex_case() {
        let json = r#"{
            "root":[
                    {
                        "id":0,
                        "name":"kai",
                        "data": {
                            "age":26,
                            "details":[
                                {
                                    "likes":["hamabe","imada"],
                                    "hobby":"rust"
                                },
                                {
                                    "userId":10000
                                }
                            ]
                        }
                    },
                    {
                        "id":0,
                        "age":25
                    },
                    {
                        "id":0,
                        "age":35,
                        "data": {
                            "age":26,
                            "from":"kanagawa",
                            "details":[
                                {
                                    "frends":["hamabe","imada"]
                                },
                                {
                                    "frendId":10000
                                }
                            ]
                        },
                        "details":[
                            {
                                "likes":["hamabe","imada"],
                                "hobby":"rust"
                            },
                            {
                                "userId":10000
                            }
                        ]
                    }
            ]
        }
        "#;
        let name = "Test";
        let kind = make_composite_type_easy(vec![(
            "root",
            make_array_type_easy(make_composite_type_easy(vec![
                ("id", type_kind_usize()),
                ("age", type_kind_usize()),
                ("name", type_kind_string()),
                (
                    "data",
                    make_composite_type_easy(vec![
                        ("age", type_kind_usize()),
                        ("from", type_kind_string()),
                        (
                            "details",
                            make_array_type_easy(make_composite_type_easy(vec![
                                ("likes", make_array_type_easy(type_kind_string())),
                                ("hobby", type_kind_string()),
                                ("userId", type_kind_usize()),
                                ("frendId", type_kind_usize()),
                                ("frends", make_array_type_easy(type_kind_string())),
                            ])),
                        ),
                    ]),
                ),
                (
                    "details",
                    make_array_type_easy(make_composite_type_easy(vec![
                        ("likes", make_array_type_easy(type_kind_string())),
                        ("hobby", type_kind_string()),
                        ("userId", type_kind_usize()),
                    ])),
                ),
            ])),
        )]);
        let tobe = TypeStructure::new(name, kind);
        let expect = TypeStructure::from_json_to_nest_type(name, Json::from(json));
        assert_eq!(expect, tobe);
    }

    #[test]
    fn test_simple_case() {
        let name = "Test";
        let expect = TypeStructure::from_json_to_nest_type(name, Json::from(r#"{"key":"value"}"#));
        let tobe = TypeStructure::new(
            name,
            make_composite_type_easy(vec![("key", type_kind_string())]),
        );
        assert_eq!(expect, tobe);
    }
    #[test]
    fn test_obj_case() {
        let name = "Test";
        let json = Json::from(r#"{"name":"kai","obj":{"id":0,"name":"kai"}}"#);
        let expect = TypeStructure::from_json_to_nest_type(name, json);
        let obj_child = make_composite_type_easy(vec![
            ("name", type_kind_string()),
            ("id", type_kind_usize()),
        ]);
        let tobe = TypeStructure::new(
            name,
            make_composite_type_easy(vec![("name", type_kind_string()), ("obj", obj_child)]),
        );
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
        let expect = TypeStructure::from_json_to_nest_type(name, Json::from(json));
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
        let tobe = TypeStructure::new(name, child);
        assert_eq!(expect, tobe);
    }
    #[test]
    fn test_primitive_array_case() {
        let name = "Test";
        let expect =
            TypeStructure::from_json_to_nest_type(name, Json::from(r#"{"key":["value"]}"#));
        let mut child = HashMap::new();
        child.insert(
            PropertyKey::from("key"),
            TypeKind::Array(Box::new(TypeKind::Primitive(PrimitiveType::String))),
        );
        let tobe = TypeStructure::new(name, TypeKind::Composite(CompositeType::new(child)));
        assert_eq!(expect, tobe);
    }
}
