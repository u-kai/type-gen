use crate::{
    json::Json,
    langs::common::primitive_type_statement_generator::PrimitiveTypeStatementGenerator,
    traits::{json_lang_mapper::JsonLangMapper, optional_checker::OptionalChecker},
};

use super::{filed_key::FiledKey, type_key::TypeKey};

/// FiledType represent filed type
/// ```
/// struct Test {
///     // usize is FiledType
///     id: usize
/// }
/// ```
pub struct FiledType(String);

impl FiledType {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
    pub fn case_obj(
        type_key: &TypeKey,
        filed_key: &FiledKey,
        mapper: &impl JsonLangMapper,
        optional_checker: &impl OptionalChecker,
    ) -> Self {
        let this = filed_key.to_type_key(type_key);
        Self(
            if optional_checker.is_optional(type_key.value(), filed_key.value()) {
                mapper.make_optional_type(this.value())
            } else {
                this.drain()
            },
        )
    }
    pub fn case_array_obj(
        type_key: &TypeKey,
        filed_key: &FiledKey,
        mapper: &impl JsonLangMapper,
        optional_checker: &impl OptionalChecker,
    ) -> Self {
        let this = filed_key.to_type_key(type_key);
        Self(
            if optional_checker.is_optional(type_key.value(), filed_key.value()) {
                mapper.make_optional_type(&mapper.make_array_type(this.value()))
            } else {
                mapper.make_array_type(&this.drain())
            },
        )
    }
    pub fn case_array_primitive(
        type_key: &TypeKey,
        filed_key: &FiledKey,
        mapper: &impl JsonLangMapper,
        optional_checker: &impl OptionalChecker,
        json: Json,
    ) -> Self {
        Self(
            PrimitiveTypeStatementGenerator::new(
                type_key.value(),
                filed_key.value(),
                mapper,
                optional_checker,
            )
            .from_json_to_array(json),
        )
    }
    pub fn case_primitive(
        type_key: &TypeKey,
        filed_key: &FiledKey,
        mapper: &impl JsonLangMapper,
        optional_checker: &impl OptionalChecker,
        json: Json,
    ) -> Self {
        println!("type_key = {:#?}", type_key);
        println!("filed_key = {:#?}", filed_key);
        Self(
            PrimitiveTypeStatementGenerator::new(
                type_key.value(),
                filed_key.value(),
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

mod test_filed_type {
    use crate::{
        json::Json,
        langs::common::{
            optional_checker::BaseOptionalChecker,
            primitive_type_statement_generator::FakeMapper,
            type_define_generators::{filed_key::FiledKey, type_key::TypeKey},
        },
    };

    use super::FiledType;

    #[test]
    fn test_case_array_primitive_filed_type() {
        let type_key = TypeKey::new("Test");
        let mapper = FakeMapper;
        let optional_checker = BaseOptionalChecker::default();
        let filed_type = FiledType::case_array_primitive(
            &type_key,
            &FiledKey::new("name"),
            &mapper,
            &optional_checker,
            Json::String("".to_string()),
        );
        assert_eq!(filed_type.value(), "Option<Vec<String>>");
    }
    #[test]
    fn test_case_primitive_filed_type() {
        let type_key = TypeKey::new("Test");
        let mapper = FakeMapper;
        let optional_checker = BaseOptionalChecker::default();
        let filed_type = FiledType::case_primitive(
            &type_key,
            &FiledKey::new("name"),
            &mapper,
            &optional_checker,
            Json::String("".to_string()),
        );
        assert_eq!(filed_type.value(), "Option<String>");
    }
    #[test]
    fn test_case_array_obj_filed_type() {
        let type_key = TypeKey::new("Test");
        let mapper = FakeMapper;
        let optional_checker = BaseOptionalChecker::default();
        let filed_type =
            FiledType::case_array_obj(&type_key, &FiledKey::new("obj"), &mapper, &optional_checker);
        assert_eq!(filed_type.value(), "Option<Vec<TestObj>>");
    }
    #[test]
    fn test_case_obj_filed_type() {
        let type_key = TypeKey::new("Test");
        let mapper = FakeMapper;
        let optional_checker = BaseOptionalChecker::default();
        let filed_type =
            FiledType::case_obj(&type_key, &FiledKey::new("obj"), &mapper, &optional_checker);
        assert_eq!(filed_type.value(), "Option<TestObj>");
    }
}
