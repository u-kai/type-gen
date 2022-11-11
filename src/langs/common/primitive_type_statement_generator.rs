use crate::traits::{json_lang_mapper::JsonLangMapper, optional_checker::OptionalChecker};

pub struct PrimitiveTypeStatementGenerator<'a, M, O>
where
    M: JsonLangMapper,
    O: OptionalChecker,
{
    mapper: &'a M,
    optional_checker: &'a O,
}

impl<'a, M, O> PrimitiveTypeStatementGenerator<'a, M, O>
where
    M: JsonLangMapper,
    O: OptionalChecker,
{
    pub fn new(mapper: &'a M, optional_checker: &'a O) -> Self {
        Self {
            mapper,
            optional_checker,
        }
    }
    fn case_string(&self, type_key: &str, filed_key: &str) -> String {
        if self.optional_checker.is_optional(type_key, filed_key) {
            self.mapper.make_optional_type(self.mapper.case_string())
        } else {
            self.mapper.case_string().to_string()
        }
    }
}

#[cfg(test)]

mod test_primitive_type_statement_generator {
    use std::collections::HashMap;

    use super::*;
    struct FakeOptionalChecker {
        type_keys: Vec<&'static str>,
        field_keys: Vec<&'static str>,
    }
    impl OptionalChecker for FakeOptionalChecker {
        fn is_optional(&self, type_key: &str, filed_key: &str) -> bool {
            self.type_keys.contains(&type_key) && self.field_keys.contains(&filed_key)
        }
    }
    struct FakeMapper;
    impl JsonLangMapper for FakeMapper {
        fn case_bool(&self) -> &'static str {
            "bool"
        }
        fn case_null(&self) -> &'static str {
            "null"
        }
        fn case_num(&self, num: &serde_json::Number) -> String {
            "num".to_string()
        }
        fn case_string(&self) -> &'static str {
            "string"
        }
        fn make_array_type(&self, type_str: &str) -> String {
            format!("Vec<{}>", type_str)
        }
        fn make_optional_type(&self, type_str: &str) -> String {
            format!("Option<{}>", type_str)
        }
    }

    #[test]
    fn test_case_string() {
        let mapper = FakeMapper;
        let type_key = "Test";
        let optional_filed_key = "test";
        let require_filed_key = "id";
        let optional_checker = FakeOptionalChecker {
            type_keys: vec![type_key],
            field_keys: vec![optional_filed_key],
        };
        let generator = PrimitiveTypeStatementGenerator::new(&mapper, &optional_checker);
        assert_eq!(
            generator.case_string(type_key, optional_filed_key),
            "Option<string>".to_string()
        );
        assert_eq!(
            generator.case_string(type_key, require_filed_key),
            "string".to_string()
        );
    }
}
