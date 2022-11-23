use lang_common::types::{
    primitive_type::primitive_type_factories::{make_float, make_isize, make_usize},
    property_type::property_type_factories::{make_any, make_primitive_type},
    structures::TypeStructure,
    type_name::TypeName,
};

use crate::json::Json;

impl Json {
    pub fn into_type_structures(self, root_name: impl Into<TypeName>) -> Vec<TypeStructure> {
        match self {
            Json::Object(obj) => {
                vec![]
            }
            Json::Array(arr) => {
                vec![]
            }
            Json::String(_) => {
                vec![]
            }
            Json::Null => vec![TypeStructure::make_primitive(root_name, make_any())],
            Json::Number(num) => {
                if num.is_f64() {
                    return vec![TypeStructure::make_primitive(
                        root_name,
                        make_primitive_type(make_float()),
                    )];
                }
                if num.is_u64() {
                    return vec![TypeStructure::make_primitive(
                        root_name,
                        make_primitive_type(make_usize()),
                    )];
                }
                vec![TypeStructure::make_primitive(
                    root_name,
                    make_primitive_type(make_isize()),
                )]
            }
            Json::Boolean(_) => vec![],
        }
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
