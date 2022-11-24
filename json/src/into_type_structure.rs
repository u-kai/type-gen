use std::collections::{BTreeMap, VecDeque};

use lang_common::types::{
    primitive_type::primitive_type_factories::{
        make_bool, make_float, make_isize, make_string, make_usize,
    },
    property_key::PropertyKey,
    property_type::{
        property_type_factories::{make_any, make_primitive_type},
        PropertyType,
    },
    structures::{CompositeTypeStructure, TypeStructure},
    type_name::TypeName,
};

use crate::json::{Json, Number};

// into type structures impl

impl Json {
    pub fn into_type_structures(self, root_name: impl Into<TypeName>) -> Vec<TypeStructure> {
        let root_name = root_name.into();
        match self {
            Json::Object(obj) => Self::case_obj(&root_name, obj).into(),
            Json::Array(arr) => Self::case_arr(arr),
            Json::String(_) => Self::case_alias_string(root_name),
            Json::Null => Self::case_alias_null(root_name),
            Json::Number(num) => Self::case_alias_num(root_name, num),
            Json::Boolean(_) => Self::case_alias_boolean(root_name),
        }
    }
    fn case_obj(
        // Test
        type_name: &TypeName,
        // {id: usize, name: string, child:{id: usize,data:{name:string}}}
        obj: BTreeMap<String, Self>,
    ) -> VecDeque<TypeStructure> {
        let mut result = VecDeque::new();
        // tobe {id: usize, name:string, child:TestChild, }
        let mut properties = BTreeMap::new();
        for (key, json) in obj {
            // id,name,child
            let property_key = PropertyKey::from(key);
            match json {
                Json::String(_) => {
                    properties.insert(property_key, make_primitive_type(make_string()));
                }
                Json::Number(num) => {
                    properties.insert(property_key, Self::json_num_to_property_type(num));
                }
                Json::Boolean(_) => {
                    properties.insert(property_key, make_primitive_type(make_bool()));
                }
                Json::Null => {
                    properties.insert(property_key, make_any());
                }
                Json::Object(obj) => {
                    // TestChild
                    let type_name = property_key.to_type_name(&type_name);
                    // vec![TestChild { id:usize, data:TestChildData },TestChildData {name: string}]
                    let mut childrens = Self::case_obj(&type_name, obj);
                    // PropertyType::Custom(TestChild)
                    let property_type = PropertyType::new_custom_type(type_name);
                    result.append(&mut childrens);
                    //insert child TestChild
                    properties.insert(property_key, property_type);
                }
                Json::Array(_) => todo!(),
            }
        }
        // Test {id: usize, name: string, child: TestChild}
        let type_structure =
            TypeStructure::Composite(CompositeTypeStructure::new(type_name, properties));
        result.push_front(type_structure);

        // vec![
        //    Test {id: usize, name: string, child: TestChild},
        //    TestChild { id:usize, data:TestChildData },
        //    TestChildData {name: string}
        // ]
        result
    }
    fn case_arr(arr: Vec<Self>) -> Vec<TypeStructure> {
        vec![]
    }
    fn case_alias_string(root_name: impl Into<TypeName>) -> Vec<TypeStructure> {
        vec![TypeStructure::make_alias(
            root_name,
            make_primitive_type(make_string()),
        )]
    }
    fn case_alias_null(root_name: impl Into<TypeName>) -> Vec<TypeStructure> {
        vec![TypeStructure::make_alias(root_name, make_any())]
    }
    fn case_alias_boolean(root_name: impl Into<TypeName>) -> Vec<TypeStructure> {
        vec![TypeStructure::make_alias(
            root_name,
            make_primitive_type(make_bool()),
        )]
    }
    fn case_alias_num(root_name: impl Into<TypeName>, num: Number) -> Vec<TypeStructure> {
        vec![TypeStructure::make_alias(
            root_name,
            Self::json_num_to_property_type(num),
        )]
    }
    fn json_num_to_property_type(num: Number) -> PropertyType {
        if num.is_f64() {
            return make_primitive_type(make_float());
        }
        if num.is_u64() {
            return make_primitive_type(make_usize());
        }
        make_primitive_type(make_isize())
    }
}

#[cfg(test)]
mod test_into_type_structures {
    use lang_common::types::{
        primitive_type::primitive_type_factories::make_string,
        property_type::property_type_factories::{make_custom_type, make_primitive_type},
    };

    use super::*;
    #[test]
    fn test_nest_obj_case() {
        let name = "Test";
        let json = Json::from(r#"{"name":"kai","obj":{"id":0,"name":"kai"}}"#);
        let tobe = vec![
            TypeStructure::make_composite(
                name,
                vec![
                    ("name", make_primitive_type(make_string())),
                    ("obj", make_custom_type("TestObj")),
                ],
            ),
            TypeStructure::make_composite(
                "TestObj",
                vec![
                    ("id", make_primitive_type(make_usize())),
                    ("name", make_primitive_type(make_string())),
                ],
            ),
        ];
        let expect = json.into_type_structures(name);
        println!("expect : {:#?}", expect);
        println!("tobe : {:#?}", tobe);
        assert_eq!(expect, tobe);
    }
    #[test]
    fn test_simple_case() {
        let name = "Test";
        let json = Json::from(r#"{"key":"value"}"#);
        let tobe = vec![TypeStructure::make_composite(
            name,
            vec![("key", make_primitive_type(make_string()))],
        )];
        assert_eq!(json.into_type_structures(name), tobe);
    }
}
