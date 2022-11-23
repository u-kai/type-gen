use lang_common::type_defines::{
    additional_defines::additional_statement::{AdditionalStatement, AdditionalStatementProvider},
    generators::{generator::TypeStatementGenerator, mapper::LangTypeMapper},
};

use super::{
    additional_statements::{RustComment, RustVisibility},
    mapper::RustLangMapper,
};

pub struct RustTypeStatementGenerator {}
impl RustTypeStatementGenerator {
    pub fn new() -> Self {
        Self {}
    }
}
impl<'a>
    TypeStatementGenerator<
        RustLangMapper,
        AdditionalStatementProvider<'a, RustVisibility, RustComment<'a>>,
    > for RustTypeStatementGenerator
{
    const TYPE_PREFIX: &'static str = "struct";
    fn generate_case_primitive(
        &self,
        primitive_type: &lang_common::types::structures::PrimitiveTypeStructure,
        mapper: &RustLangMapper,
        additional_statement: &AdditionalStatementProvider<'a, RustVisibility, RustComment<'a>>,
    ) -> String {
        let mut additional = String::new();
        if let Some(comment) = additional_statement.get_type_comment(&primitive_type.name) {
            additional += &comment;
        };
        format!(
            "{additional}type {name} = {type_str};",
            additional = additional,
            name = primitive_type.name.as_str(),
            type_str = mapper.case_primitive(&primitive_type.primitive_type)
        )
    }
    fn generate_case_composite(
        &self,
        type_name: &lang_common::types::type_name::TypeName,
        properties_statement: String,
        additional_statement: &AdditionalStatementProvider<'a, RustVisibility, RustComment<'a>>,
    ) -> String {
        let mut result = String::new();
        if let Some(comment) = additional_statement.get_type_comment(type_name) {
            result += &comment;
        };
        if let Some(attribute) = additional_statement.get_type_attribute(type_name) {
            result += &attribute;
        };
        format!(
            "{}{} {} {{\n{}\n}}",
            result,
            Self::TYPE_PREFIX,
            type_name.as_str(),
            properties_statement
        )
    }
}

#[cfg(test)]
mod test_rust_type_statement_generator {
    use lang_common::{
        type_defines::{
            additional_defines::additional_statement::AdditionalStatementProvider,
            generators::generator::TypeStatementGenerator,
        },
        types::{
            primitive_type::primitive_type_factories::make_string,
            structures::PrimitiveTypeStructure, type_name::TypeName,
        },
    };

    use crate::rust::{additional_statements::RustComment, mapper::RustLangMapper};

    use super::RustTypeStatementGenerator;

    #[test]
    fn test_case_custum_with_comment() {
        let mut additional_provider = AdditionalStatementProvider::with_default_optional(false);
        let mut comment = RustComment::new();
        let comment1 = "this is comment1";
        let comment2 = "this is comment2";
        let type_name: TypeName = "Test".into();
        comment.add_comment_line(comment1);
        comment.add_comment_line(comment2);
        additional_provider.add_type_comment(&type_name, comment);
        let generator = RustTypeStatementGenerator::new();
        let tobe = r#"// this is comment1
// this is comment2
struct Test {
    id: usize,
}"#;
        assert_eq!(
            generator.generate_case_composite(
                &type_name,
                format!("    id: usize,"),
                &additional_provider
            ),
            tobe
        );
    }
    #[test]
    fn test_case_custum_all_none_additional() {
        let additional_provider = AdditionalStatementProvider::with_default_optional(false);
        let type_name: TypeName = "Test".into();
        let generator = RustTypeStatementGenerator::new();
        let tobe = r#"struct Test {
    id: usize,
}"#;
        assert_eq!(
            generator.generate_case_composite(
                &type_name,
                format!("    id: usize,"),
                &additional_provider
            ),
            tobe
        );
    }
    #[test]
    fn test_case_array_option_primitive_all_none_additional() {
        let mut additional_provider = AdditionalStatementProvider::with_default_optional(false);
        let mut comment = RustComment::new();
        let comment1 = "this is comment1";
        let comment2 = "this is comment2";
        comment.add_comment_line(comment1);
        comment.add_comment_line(comment2);
        let type_name: TypeName = "Test".into();
        additional_provider.add_type_comment(&type_name, comment);
        let mapper = RustLangMapper;
        let primitive_type = PrimitiveTypeStructure::new(type_name.clone(), make_string());
        let generator = RustTypeStatementGenerator::new();
        let tobe = format!("// {comment1}\n// {comment2}\ntype Test = String;");
        assert_eq!(
            generator.generate_case_primitive(&primitive_type, &mapper, &additional_provider),
            tobe
        );
    }
    #[test]
    fn test_case_primitive_all_none_additional() {
        let additional_provider = AdditionalStatementProvider::with_default_optional(false);
        let type_name: TypeName = "Test".into();
        let mapper = RustLangMapper;
        let primitive_type = PrimitiveTypeStructure::new(type_name, make_string());
        let generator = RustTypeStatementGenerator::new();
        let tobe = format!("type Test = String;");
        assert_eq!(
            generator.generate_case_primitive(&primitive_type, &mapper, &additional_provider),
            tobe
        );
    }
}
