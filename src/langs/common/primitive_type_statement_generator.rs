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
    pub fn case_num_array(
        &self,
        type_key: &str,
        filed_key: &str,
        num: &serde_json::Number,
    ) -> String {
        let array_type = self
            .mapper
            .make_array_type(self.mapper.case_num(num).as_str());
        if self.optional_checker.is_optional(type_key, filed_key) {
            self.mapper.make_optional_type(&array_type)
        } else {
            array_type
        }
    }
    pub fn case_null_array(&self, type_key: &str, filed_key: &str) -> String {
        let array_type = self.mapper.make_array_type(self.mapper.case_null());
        if self.optional_checker.is_optional(type_key, filed_key) {
            self.mapper.make_optional_type(&array_type)
        } else {
            array_type
        }
    }

    pub fn case_boolean_array(&self, type_key: &str, filed_key: &str) -> String {
        let array_type = self.mapper.make_array_type(self.mapper.case_bool());
        if self.optional_checker.is_optional(type_key, filed_key) {
            self.mapper.make_optional_type(&array_type)
        } else {
            array_type
        }
    }
    pub fn case_string_array(&self, type_key: &str, filed_key: &str) -> String {
        let array_type = self.mapper.make_array_type(self.mapper.case_string());
        if self.optional_checker.is_optional(type_key, filed_key) {
            self.mapper.make_optional_type(&array_type)
        } else {
            array_type
        }
    }
    pub fn case_boolean(&self, type_key: &str, filed_key: &str) -> String {
        if self.optional_checker.is_optional(type_key, filed_key) {
            self.mapper.make_optional_type(self.mapper.case_bool())
        } else {
            self.mapper.case_bool().to_string()
        }
    }
    pub fn case_num(&self, type_key: &str, filed_key: &str, num: &serde_json::Number) -> String {
        if self.optional_checker.is_optional(type_key, filed_key) {
            self.mapper.make_optional_type(&self.mapper.case_num(&num))
        } else {
            self.mapper.case_num(&num)
        }
    }
    pub fn case_null(&self, type_key: &str, filed_key: &str) -> String {
        if self.optional_checker.is_optional(type_key, filed_key) {
            self.mapper.make_optional_type(self.mapper.case_null())
        } else {
            self.mapper.case_null().to_string()
        }
    }
    pub fn case_string(&self, type_key: &str, filed_key: &str) -> String {
        if self.optional_checker.is_optional(type_key, filed_key) {
            self.mapper.make_optional_type(self.mapper.case_string())
        } else {
            self.mapper.case_string().to_string()
        }
    }
}

#[cfg(test)]

mod test_primitive_type_statement_generator {
    use super::*;
    use serde_json::Number;
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
        fn case_num(&self, _: &serde_json::Number) -> String {
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
    fn test_case_bool_array() {
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
            generator.case_boolean_array(type_key, optional_filed_key,),
            "Option<Vec<bool>>".to_string()
        );
        assert_eq!(
            generator.case_boolean_array(type_key, require_filed_key,),
            "Vec<bool>".to_string()
        );
    }
    #[test]
    fn test_case_num_array() {
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
            generator.case_num_array(
                type_key,
                optional_filed_key,
                &Number::from_f64(0_f64).unwrap()
            ),
            "Option<Vec<num>>".to_string()
        );
        assert_eq!(
            generator.case_num_array(
                type_key,
                require_filed_key,
                &Number::from_f64(0_f64).unwrap()
            ),
            "Vec<num>".to_string()
        );
    }
    #[test]
    fn test_case_null_array() {
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
            generator.case_null_array(type_key, optional_filed_key),
            "Option<Vec<null>>".to_string()
        );
        assert_eq!(
            generator.case_null_array(type_key, require_filed_key),
            "Vec<null>".to_string()
        );
    }
    #[test]
    fn test_case_string_array() {
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
            generator.case_string_array(type_key, optional_filed_key),
            "Option<Vec<string>>".to_string()
        );
        assert_eq!(
            generator.case_string_array(type_key, require_filed_key),
            "Vec<string>".to_string()
        );
    }
    #[test]
    fn test_case_num() {
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
            generator.case_num(
                type_key,
                optional_filed_key,
                &Number::from_f64(0_f64).unwrap()
            ),
            "Option<num>".to_string()
        );
        assert_eq!(
            generator.case_num(
                type_key,
                require_filed_key,
                &Number::from_f64(0_f64).unwrap()
            ),
            "num".to_string()
        );
    }
    #[test]
    fn test_case_null() {
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
            generator.case_null(type_key, optional_filed_key),
            "Option<null>".to_string()
        );
        assert_eq!(
            generator.case_null(type_key, require_filed_key),
            "null".to_string()
        );
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
