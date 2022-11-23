use std::collections::BTreeMap;

use lang_common::types::{
    primitive_type::primitive_type_factories::{
        make_bool, make_float, make_isize, make_string, make_usize,
    },
    property_type::property_type_factories::{make_any, make_primitive_type},
    structures::TypeStructure,
    type_name::TypeName,
};

use crate::json::{Json, Number};

// into type structures impl

impl Json {
    pub fn into_type_structures(self, root_name: impl Into<TypeName>) -> Vec<TypeStructure> {
        match self {
            Json::Object(obj) => Self::case_obj(obj),
            Json::Array(arr) => Self::case_arr(arr),
            Json::String(_) => Self::case_alias_string(root_name),
            Json::Null => vec![TypeStructure::make_alias(root_name, make_any())],
            Json::Number(num) => Self::case_alias_num(root_name, num),
            Json::Boolean(_) => Self::case_alias_boolean(root_name),
        }
    }
    fn case_obj(obj: BTreeMap<String, Self>) -> Vec<TypeStructure> {
        vec![]
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
    fn case_alias_boolean(root_name: impl Into<TypeName>) -> Vec<TypeStructure> {
        vec![TypeStructure::make_alias(
            root_name,
            make_primitive_type(make_bool()),
        )]
    }
    fn case_alias_num(root_name: impl Into<TypeName>, num: Number) -> Vec<TypeStructure> {
        if num.is_f64() {
            return vec![TypeStructure::make_alias(
                root_name,
                make_primitive_type(make_float()),
            )];
        }
        if num.is_u64() {
            return vec![TypeStructure::make_alias(
                root_name,
                make_primitive_type(make_usize()),
            )];
        }
        vec![TypeStructure::make_alias(
            root_name,
            make_primitive_type(make_isize()),
        )]
    }
}

#[cfg(test)]
mod test_into_type_structures {
    use lang_common::types::{
        primitive_type::primitive_type_factories::make_string,
        property_type::property_type_factories::make_primitive_type,
    };

    use super::*;
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
