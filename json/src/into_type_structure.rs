use std::collections::{BTreeMap, VecDeque};

use structure::{
    alias_type_structure::AliasTypeStructure,
    composite_type_structure::CompositeTypeStructure,
    parts::{
        property_key::PropertyKey,
        property_type::{
            property_type_factories::{
                make_any, make_array_type, make_bool_type, make_custom_type, make_float_type,
                make_isize_type, make_string_type, make_usize_type,
            },
            PropertyType,
        },
        type_name::TypeName,
    },
    type_structure::TypeStructure,
};

use crate::json::{Json, JsonType, Number};

// into type structures impl
impl Json {
    pub fn into_type_structures(self, root_name: impl Into<TypeName>) -> Vec<TypeStructure> {
        let root_name = root_name.into();
        match self {
            Json::Object(obj) => Self::case_obj(&root_name, obj).into(),
            Json::Array(arr) => Self::case_alias_arr(&root_name, arr).into(),
            Json::String(_) => Self::case_alias_string(root_name),
            Json::Null => Self::case_alias_null(root_name),
            Json::Number(num) => Self::case_alias_num(root_name, num),
            Json::Boolean(_) => Self::case_alias_boolean(root_name),
        }
    }
    fn case_obj(
        // Test
        type_name: &TypeName,
        // {id: usize, name: string, child:{id: usize,data:{name:string}},arr:[{id:usize},{name:string}]}
        obj: BTreeMap<String, Self>,
    ) -> VecDeque<TypeStructure> {
        let mut result = VecDeque::new();
        // tobe {id: usize, name:string, child:TestChild, }
        let mut properties = BTreeMap::new();
        for (key, json) in obj {
            // id,name,child
            let property_key = PropertyKey::from(key);
            // TestChild
            let type_name = property_key.to_type_name(&type_name);
            match json {
                Json::Object(obj) => {
                    // vec![TestChild { id:usize, data:TestChildData },TestChildData {name: string}]
                    let mut childrens = Self::case_obj(&type_name, obj);
                    // PropertyType::Custom(TestChild)
                    let property_type = Self::obj_property_type(type_name);
                    result.append(&mut childrens);
                    //insert child TestChild
                    properties.insert(property_key, property_type);
                }
                Json::Array(arr) => {
                    let put_together = Self::put_together(arr);
                    let property_type = Self::array_property_type(&type_name, &put_together);
                    properties.insert(property_key, property_type);
                    let mut childrens = Self::put_togeher_to_structures(&type_name, put_together);
                    result.append(&mut childrens);
                }
                _ => {
                    properties.insert(property_key, Self::into_primitive_property_type(&json));
                }
            }
        }
        // Test {id: usize, name: string, child: TestChild}
        // case obj is empty
        let type_structure = if properties.len() == 0 {
            TypeStructure::Alias(AliasTypeStructure::new(type_name, PropertyType::Any))
        } else {
            TypeStructure::Composite(CompositeTypeStructure::new(type_name, properties))
        };
        result.push_front(type_structure);
        // vec![
        //    Test {id: usize, name: string, child: TestChild},
        //    TestChild { id:usize, data:TestChildData },
        //    TestChildData {name: string}
        // ]
        result
    }
    fn case_alias_arr(type_name: &TypeName, array: Vec<Json>) -> VecDeque<TypeStructure> {
        let put_together = Self::put_together(array);
        let mut result = Self::put_togeher_to_structures(type_name, put_together);
        result.push_front(TypeStructure::make_alias(
            format!("{}Array", type_name.as_str()),
            make_array_type(make_custom_type(type_name)),
        ));
        result
    }
    fn put_togeher_to_structures(
        type_name: &TypeName,
        put_together: [Json; 1],
    ) -> VecDeque<TypeStructure> {
        put_together
            .into_iter()
            .flat_map(|json| match json {
                Json::Object(obj) => Self::case_obj(type_name, obj),
                Json::Array(_) => json.into_type_structures(type_name).into(),
                _ => VecDeque::new(),
            })
            .collect::<VecDeque<_>>()
    }
    fn case_alias_string(root_name: impl Into<TypeName>) -> Vec<TypeStructure> {
        vec![TypeStructure::make_alias(root_name, make_string_type())]
    }
    fn case_alias_null(root_name: impl Into<TypeName>) -> Vec<TypeStructure> {
        vec![TypeStructure::make_alias(root_name, make_any())]
    }
    fn case_alias_boolean(root_name: impl Into<TypeName>) -> Vec<TypeStructure> {
        vec![TypeStructure::make_alias(root_name, make_bool_type())]
    }
    fn case_alias_num(root_name: impl Into<TypeName>, num: Number) -> Vec<TypeStructure> {
        vec![TypeStructure::make_alias(
            root_name,
            Self::json_num_to_property_type(&num),
        )]
    }
    fn array_property_type(type_name: &TypeName, put_together: &[Json; 1]) -> PropertyType {
        let nest_num = Self::count_put_together_nest(put_together);
        let json_type = Self::put_together_content_type(put_together);
        let property_type = match json_type {
            JsonType::Boolean => make_bool_type(),
            JsonType::Float64 => make_float_type(),
            JsonType::Usize64 => make_usize_type(),
            JsonType::Isize64 => make_isize_type(),
            JsonType::String => make_string_type(),
            JsonType::Object => make_custom_type(type_name),
            JsonType::Null => make_any(),
            JsonType::Array => panic!(),
        };
        (0..nest_num).fold(property_type, |acc, _| acc.to_array())
    }
    fn obj_property_type(type_name: impl Into<TypeName>) -> PropertyType {
        PropertyType::new_custom_type(type_name)
    }
    fn into_primitive_property_type(&self) -> PropertyType {
        match self {
            Json::String(_) => make_string_type(),
            Json::Number(num) => Self::json_num_to_property_type(num),
            Json::Boolean(_) => make_bool_type(),
            Json::Null => make_any(),
            _ => panic!("not use not primitive type! json is {:#?}", self),
        }
    }
    fn json_num_to_property_type(num: &Number) -> PropertyType {
        if num.is_f64() {
            return make_float_type();
        }
        if num.is_i64() {
            return make_isize_type();
        }
        make_usize_type()
    }
}

#[cfg(test)]
mod test_into_type_structures {

    use super::*;
    #[test]
    fn test_case_only_array() {
        let json = Json::from(r#"[{"key":"value"}]"#);
        assert_eq!(
            json.into_type_structures("Test"),
            vec![
                TypeStructure::make_alias("TestArray", make_array_type(make_custom_type("Test"))),
                TypeStructure::make_composite("Test", vec![("key", make_string_type())])
            ]
        );
    }
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
        let test = TypeStructure::make_composite(
            "Test",
            vec![("root", make_array_type(make_custom_type("TestRoot")))],
        );
        let test_root = TypeStructure::make_composite(
            "TestRoot",
            vec![
                ("id", make_usize_type()),
                ("age", make_usize_type()),
                ("name", make_string_type()),
                ("data", make_custom_type("TestRootData")),
                (
                    "details",
                    make_array_type(make_custom_type("TestRootDetails")),
                ),
            ],
        );
        let test_root_data = TypeStructure::make_composite(
            "TestRootData",
            vec![
                ("age", make_usize_type()),
                ("from", make_string_type()),
                (
                    "details",
                    make_array_type(make_custom_type("TestRootDataDetails")),
                ),
            ],
        );
        let test_root_details = TypeStructure::make_composite(
            "TestRootDetails",
            vec![
                ("likes", make_array_type(make_string_type())),
                ("hobby", make_string_type()),
                ("userId", make_usize_type()),
            ],
        );
        let test_root_data_details = TypeStructure::make_composite(
            "TestRootDataDetails",
            vec![
                ("likes", make_array_type(make_string_type())),
                ("hobby", make_string_type()),
                ("userId", make_usize_type()),
                ("frendId", make_usize_type()),
                ("frends", make_array_type(make_string_type())),
            ],
        );
        let tobe = vec![
            test,
            test_root,
            test_root_data,
            test_root_data_details,
            test_root_details,
        ];
        let expect = Json::from(json).into_type_structures("Test");
        println!("tobe {:#?}", tobe);
        println!("expect {:#?}", expect);
        assert_eq!(expect, tobe);
    }
    #[test]
    fn test_nest_obj_case() {
        let name = "Test";
        let json = Json::from(r#"{"name":"kai","obj":{"id":0,"name":"kai"}}"#);
        let tobe = vec![
            TypeStructure::make_composite(
                name,
                vec![
                    ("name", make_string_type()),
                    ("obj", make_custom_type("TestObj")),
                ],
            ),
            TypeStructure::make_composite(
                "TestObj",
                vec![("id", make_usize_type()), ("name", make_string_type())],
            ),
        ];
        let expect = json.into_type_structures(name);
        println!("expect : {:#?}", expect);
        println!("tobe : {:#?}", tobe);
        assert_eq!(expect, tobe);
    }
    #[test]
    fn test_simple_obj_and_arr_case() {
        let name = "Test";
        let json = Json::from(r#"{"key":"value","arr":["value1","value2"]}"#);
        let tobe = vec![TypeStructure::make_composite(
            name,
            vec![
                ("key", make_string_type()),
                ("arr", make_array_type(make_string_type())),
            ],
        )];
        assert_eq!(json.into_type_structures(name), tobe);
    }
    #[test]
    fn test_case_empty_obj() {
        let name = "Test";
        let json = Json::from(r#"{}"#);
        let tobe = vec![TypeStructure::make_alias(name, make_any())];
        assert_eq!(json.into_type_structures(name), tobe);
    }
    #[test]
    fn test_simple_obj_case() {
        let name = "Test";
        let json = Json::from(r#"{"key":"value"}"#);
        let tobe = vec![TypeStructure::make_composite(
            name,
            vec![("key", make_string_type())],
        )];
        assert_eq!(json.into_type_structures(name), tobe);
    }
}
