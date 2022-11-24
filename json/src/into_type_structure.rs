use std::collections::BTreeMap;

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
        match self {
            Json::Object(obj) => Self::case_root_obj(root_name, obj),
            Json::Array(arr) => Self::case_arr(arr),
            Json::String(_) => Self::case_alias_string(root_name),
            Json::Null => Self::case_alias_null(root_name),
            Json::Number(num) => Self::case_alias_num(root_name, num),
            Json::Boolean(_) => Self::case_alias_boolean(root_name),
        }
    }
    fn case_root_obj(
        root_name: impl Into<TypeName>,
        obj: BTreeMap<String, Self>,
    ) -> Vec<TypeStructure> {
        let root_name = root_name.into();
        let mut result = Vec::new();
        let mut root_properties = BTreeMap::new();
        for (key, json) in obj {
            let property_key = PropertyKey::from(key);
            let (property_type, children) = match json {
                Json::String(_) => (make_primitive_type(make_string()), None),
                Json::Number(num) => (Self::json_num_to_property_type(num), None),
                Json::Boolean(_) => (make_primitive_type(make_bool()), None),
                Json::Null => (make_any(), None),
                Json::Object(obj) => {
                    let type_name = property_key.to_type_name(&root_name);
                    let childrens = Self::obj_to_property_types(&type_name, obj);
                    let property_type = PropertyType::new_custom_type(type_name);
                    (property_type, Some(childrens))
                }
                Json::Array(_) => todo!(),
            };
            root_properties.insert(property_key, property_type);
            children.map(|mut children| result.append(&mut children));
        }
        let root =
            TypeStructure::Composite(CompositeTypeStructure::new(root_name, root_properties));
        result.insert(0, root);
        result
    }
    fn obj_to_property_types(
        parent_name: &TypeName,
        obj: BTreeMap<String, Self>,
    ) -> Vec<TypeStructure> {
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
