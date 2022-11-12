use crate::traits::{json_lang_mapper::JsonLangMapper, optional_checker::OptionalChecker};

pub struct PrimitiveTypeStatementGenerator<'a, M, O>
where
    M: JsonLangMapper,
    O: OptionalChecker,
{
    type_key: &'a str,
    filed_key: &'a str,
    mapper: &'a M,
    optional_checker: &'a O,
}

impl<'a, M, O> PrimitiveTypeStatementGenerator<'a, M, O>
where
    M: JsonLangMapper,
    O: OptionalChecker,
{
    pub fn new(
        type_key: &'a str,
        filed_key: &'a str,
        mapper: &'a M,
        optional_checker: &'a O,
    ) -> Self {
        Self {
            type_key,
            filed_key,
            mapper,
            optional_checker,
        }
    }
    pub fn case_num_array(&self, num: &serde_json::Number) -> String {
        let array_type = self
            .mapper
            .make_array_type(self.mapper.case_num(num).as_str());
        self.gen_optional_or_require(array_type)
    }
    pub fn case_null_array(&self) -> String {
        let array_type = self.mapper.make_array_type(self.mapper.case_null());
        self.gen_optional_or_require(array_type)
    }

    pub fn case_boolean_array(&self) -> String {
        let array_type = self.mapper.make_array_type(self.mapper.case_bool());
        self.gen_optional_or_require(array_type)
    }
    pub fn case_string_array(&self) -> String {
        let array_type = self.mapper.make_array_type(self.mapper.case_string());
        self.gen_optional_or_require(array_type)
    }
    pub fn case_boolean(&self) -> String {
        self.gen_optional_or_require(self.mapper.case_bool())
    }
    pub fn case_num(&self, num: &serde_json::Number) -> String {
        self.gen_optional_or_require(self.mapper.case_num(num))
    }
    pub fn case_null(&self) -> String {
        self.gen_optional_or_require(self.mapper.case_null())
    }
    pub fn case_string(&self) -> String {
        self.gen_optional_or_require(self.mapper.case_string())
    }
    fn gen_optional_or_require(&self, filed_type: impl Into<String>) -> String {
        let filed_type: String = filed_type.into();
        if self
            .optional_checker
            .is_optional(self.type_key, self.filed_key)
        {
            self.mapper.make_optional_type(filed_type.as_str())
        } else {
            filed_type.into()
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
        let generator = PrimitiveTypeStatementGenerator::new(
            type_key,
            optional_filed_key,
            &mapper,
            &optional_checker,
        );
        assert_eq!(
            generator.case_boolean_array(),
            "Option<Vec<bool>>".to_string()
        );
        let generator = PrimitiveTypeStatementGenerator::new(
            type_key,
            require_filed_key,
            &mapper,
            &optional_checker,
        );
        assert_eq!(generator.case_boolean_array(), "Vec<bool>".to_string());
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
        let generator = PrimitiveTypeStatementGenerator::new(
            type_key,
            optional_filed_key,
            &mapper,
            &optional_checker,
        );
        assert_eq!(
            generator.case_num_array(&Number::from_f64(0_f64).unwrap()),
            "Option<Vec<usize>>".to_string()
        );
        let generator = PrimitiveTypeStatementGenerator::new(
            type_key,
            require_filed_key,
            &mapper,
            &optional_checker,
        );
        assert_eq!(
            generator.case_num_array(&Number::from_f64(0_f64).unwrap()),
            "Vec<usize>".to_string()
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
        let generator = PrimitiveTypeStatementGenerator::new(
            type_key,
            optional_filed_key,
            &mapper,
            &optional_checker,
        );
        assert_eq!(generator.case_null_array(), "Option<Vec<null>>".to_string());
        let generator = PrimitiveTypeStatementGenerator::new(
            type_key,
            require_filed_key,
            &mapper,
            &optional_checker,
        );
        assert_eq!(generator.case_null_array(), "Vec<null>".to_string());
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
        let generator = PrimitiveTypeStatementGenerator::new(
            type_key,
            optional_filed_key,
            &mapper,
            &optional_checker,
        );
        assert_eq!(
            generator.case_string_array(),
            "Option<Vec<String>>".to_string()
        );
        let generator = PrimitiveTypeStatementGenerator::new(
            type_key,
            require_filed_key,
            &mapper,
            &optional_checker,
        );
        assert_eq!(generator.case_string_array(), "Vec<String>".to_string());
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
        let generator = PrimitiveTypeStatementGenerator::new(
            type_key,
            optional_filed_key,
            &mapper,
            &optional_checker,
        );
        assert_eq!(
            generator.case_num(&Number::from_f64(0_f64).unwrap()),
            "Option<usize>".to_string()
        );
        let generator = PrimitiveTypeStatementGenerator::new(
            type_key,
            require_filed_key,
            &mapper,
            &optional_checker,
        );
        assert_eq!(
            generator.case_num(&Number::from_f64(0_f64).unwrap()),
            "usize".to_string()
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
        let generator = PrimitiveTypeStatementGenerator::new(
            type_key,
            optional_filed_key,
            &mapper,
            &optional_checker,
        );
        assert_eq!(generator.case_null(), "Option<null>".to_string());
        let generator = PrimitiveTypeStatementGenerator::new(
            type_key,
            require_filed_key,
            &mapper,
            &optional_checker,
        );
        assert_eq!(generator.case_null(), "null".to_string());
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
        let generator = PrimitiveTypeStatementGenerator::new(
            type_key,
            optional_filed_key,
            &mapper,
            &optional_checker,
        );
        assert_eq!(generator.case_string(), "Option<String>".to_string());
        let generator = PrimitiveTypeStatementGenerator::new(
            type_key,
            require_filed_key,
            &mapper,
            &optional_checker,
        );
        assert_eq!(generator.case_string(), "String".to_string());
    }
}

pub struct FakeMapper;
impl FakeMapper {
    pub fn case_without_num(&self) -> String {
        "usize".to_string()
    }
}
impl JsonLangMapper for FakeMapper {
    fn case_bool(&self) -> &'static str {
        "bool"
    }
    fn case_null(&self) -> &'static str {
        "null"
    }
    fn case_num(&self, _: &serde_json::Number) -> String {
        self.case_without_num()
    }
    fn case_string(&self) -> &'static str {
        "String"
    }
    fn make_array_type(&self, type_str: &str) -> String {
        format!("Vec<{}>", type_str)
    }
    fn make_optional_type(&self, type_str: &str) -> String {
        format!("Option<{}>", type_str)
    }
}
