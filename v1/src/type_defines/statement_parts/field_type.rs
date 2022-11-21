use crate::{
    json::Json,
    type_defines::generators::from_json::lang_common::{
        json_lang_mapper::JsonLangMapper, optional_checker::OptionalChecker,
        primitive_type_statement_generator::PrimitiveTypeStatementGenerator,
    },
};

use super::{field_key::FieldKey, type_key::TypeKey};

/// fieldType represent field type
/// ```
/// struct Test {
///     // usize is fieldType
///     id: usize
/// }
/// ```
pub struct FieldType(String);

impl FieldType {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
    fn from_nest_array(
        nest_num: usize,
        type_field_str: impl Into<String>,
        mapper: &impl JsonLangMapper,
    ) -> String {
        (0..nest_num).fold(type_field_str.into(), |acc, _| mapper.make_array_type(&acc))
    }
    pub fn case_obj(
        type_key: &TypeKey,
        field_key: &FieldKey,
        mapper: &impl JsonLangMapper,
        optional_checker: &impl OptionalChecker,
    ) -> Self {
        let this = field_key.to_type_key(type_key);
        Self(
            if optional_checker.is_optional(type_key.value(), field_key.original()) {
                mapper.make_optional_type(this.value())
            } else {
                this.drain()
            },
        )
    }
    pub fn case_nest_array_obj(
        type_key: &TypeKey,
        field_key: &FieldKey,
        nest_num: usize,
        mapper: &impl JsonLangMapper,
        optional_checker: &impl OptionalChecker,
    ) -> Self {
        let this = field_key.to_type_key(type_key);
        Self(
            if optional_checker.is_optional(type_key.value(), field_key.original()) {
                mapper.make_optional_type(&Self::from_nest_array(nest_num, this.value(), mapper))
            } else {
                Self::from_nest_array(nest_num, this.value(), mapper)
            },
        )
    }
    pub fn case_array_obj(
        type_key: &TypeKey,
        field_key: &FieldKey,
        mapper: &impl JsonLangMapper,
        optional_checker: &impl OptionalChecker,
    ) -> Self {
        let this = field_key.to_type_key(type_key);
        Self(
            if optional_checker.is_optional(type_key.value(), field_key.original()) {
                mapper.make_optional_type(&mapper.make_array_type(this.value()))
            } else {
                mapper.make_array_type(&this.drain())
            },
        )
    }
    pub fn case_nest_array_primitive(
        type_key: &TypeKey,
        field_key: &FieldKey,
        mapper: &impl JsonLangMapper,
        optional_checker: &impl OptionalChecker,
        json: &Json,
        nest_num: usize,
    ) -> Self {
        Self(
            PrimitiveTypeStatementGenerator::new(
                type_key.value(),
                field_key.original(),
                mapper,
                optional_checker,
            )
            .from_json_to_nest_array(json, nest_num),
        )
    }
    pub fn case_array_primitive(
        type_key: &TypeKey,
        field_key: &FieldKey,
        mapper: &impl JsonLangMapper,
        optional_checker: &impl OptionalChecker,
        json: Json,
    ) -> Self {
        Self(
            PrimitiveTypeStatementGenerator::new(
                type_key.value(),
                field_key.original(),
                mapper,
                optional_checker,
            )
            .from_json_to_array(json),
        )
    }
    pub fn case_primitive(
        type_key: &TypeKey,
        field_key: &FieldKey,
        mapper: &impl JsonLangMapper,
        optional_checker: &impl OptionalChecker,
        json: Json,
    ) -> Self {
        Self(
            PrimitiveTypeStatementGenerator::new(
                type_key.value(),
                field_key.original(),
                mapper,
                optional_checker,
            )
            .from_json(json),
        )
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]

mod test_field_type {
    use crate::{
        json::Json,
        type_defines::{
            generators::from_json::lang_common::{
                optional_checker::BaseOptionalChecker,
                primitive_type_statement_generator::FakeMapper,
            },
            statement_parts::{field_key::FieldKey, type_key::TypeKey},
        },
    };

    use super::FieldType;

    #[test]
    fn test_case_nest_array_obj_field_type() {
        let type_key = TypeKey::new("Test");
        let mapper = FakeMapper;
        let optional_checker = BaseOptionalChecker::default();
        let field_type = FieldType::case_nest_array_obj(
            &type_key,
            &FieldKey::new("name"),
            3,
            &mapper,
            &optional_checker,
        );
        assert_eq!(field_type.value(), "Option<Vec<Vec<Vec<TestName>>>>");
    }
    #[test]
    fn test_case_array_primitive_field_type() {
        let type_key = TypeKey::new("Test");
        let mapper = FakeMapper;
        let optional_checker = BaseOptionalChecker::default();
        let field_type = FieldType::case_array_primitive(
            &type_key,
            &FieldKey::new("name"),
            &mapper,
            &optional_checker,
            Json::String("".to_string()),
        );
        assert_eq!(field_type.value(), "Option<Vec<String>>");
    }
    #[test]
    fn test_case_primitive_field_type() {
        let type_key = TypeKey::new("Test");
        let mapper = FakeMapper;
        let optional_checker = BaseOptionalChecker::default();
        let field_type = FieldType::case_primitive(
            &type_key,
            &FieldKey::new("name"),
            &mapper,
            &optional_checker,
            Json::String("".to_string()),
        );
        assert_eq!(field_type.value(), "Option<String>");
    }
    #[test]
    fn test_case_array_obj_field_type() {
        let type_key = TypeKey::new("Test");
        let mapper = FakeMapper;
        let optional_checker = BaseOptionalChecker::default();
        let field_type =
            FieldType::case_array_obj(&type_key, &FieldKey::new("obj"), &mapper, &optional_checker);
        assert_eq!(field_type.value(), "Option<Vec<TestObj>>");
    }
    #[test]
    fn test_case_obj_field_type() {
        let type_key = TypeKey::new("Test");
        let mapper = FakeMapper;
        let optional_checker = BaseOptionalChecker::default();
        let field_type =
            FieldType::case_obj(&type_key, &FieldKey::new("obj"), &mapper, &optional_checker);
        assert_eq!(field_type.value(), "Option<TestObj>");
    }
}
