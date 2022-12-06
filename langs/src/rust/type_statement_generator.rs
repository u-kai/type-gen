use lang_common::{
    type_defines::{
        additional_defines::additional_statement::{
            AdditionalStatement, AdditionalStatementProvider,
        },
        generators::{generator::TypeStatementGenerator, mapper::LangTypeMapper},
    },
    types::type_name::TypeName,
};

use super::{
    additional_statements::{RustComment, RustVisibility},
    attribute::RustAttribute,
    mapper::RustLangMapper,
    reserved_words::replace_cannot_use_char,
};

pub struct RustTypeStatementGenerator {}
impl<'a> RustTypeStatementGenerator {
    pub fn new() -> Self {
        Self {}
    }
    fn make_additional(
        &self,
        type_name: &TypeName,
        additional_provider: &AdditionalStatementProvider<
            RustVisibility,
            RustComment,
            RustAttribute,
        >,
    ) -> String {
        let mut result = String::new();
        if let Some(comment) = additional_provider.get_type_comment(type_name) {
            result += &comment;
        };
        if let Some(attribute) = additional_provider.get_type_attribute(type_name) {
            result += &attribute;
        };
        result += additional_provider.get_type_visibility(type_name);
        result
    }
}
impl<'a>
    TypeStatementGenerator<
        RustLangMapper,
        AdditionalStatementProvider<RustVisibility, RustComment, RustAttribute>,
    > for RustTypeStatementGenerator
{
    const TYPE_PREFIX: &'static str = "struct";
    fn generate_case_primitive(
        &self,
        primitive_type: &lang_common::types::structures::AliasTypeStructure,
        mapper: &RustLangMapper,
        additional_statement: &AdditionalStatementProvider<
            RustVisibility,
            RustComment,
            RustAttribute,
        >,
    ) -> String {
        let additional = self.make_additional(&primitive_type.name, additional_statement);
        format!(
            "{additional}type {name} = {type_str};",
            additional = additional,
            name = replace_cannot_use_char(primitive_type.name.as_str()),
            type_str = mapper.case_property_type(&primitive_type.property_type)
        )
    }
    fn generate_case_composite(
        &self,
        type_name: &lang_common::types::type_name::TypeName,
        properties_statement: String,
        additional_statement: &AdditionalStatementProvider<
            RustVisibility,
            RustComment,
            RustAttribute,
        >,
    ) -> String {
        let additional = self.make_additional(type_name, additional_statement);
        format!(
            "{}{} {} {{\n{}}}",
            additional,
            Self::TYPE_PREFIX,
            replace_cannot_use_char(type_name.as_str()),
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
            property_type::property_type_factories::make_primitive_type,
            structures::AliasTypeStructure, type_name::TypeName,
        },
    };

    use crate::rust::{
        additional_statements::{RustComment, RustVisibility},
        attribute::{RustAttribute, RustAttributeKind},
        mapper::RustLangMapper,
    };

    use super::RustTypeStatementGenerator;

    #[test]
    fn test_case_custum_set_all() {
        let type_name = "Test";
        let mut additional_provider = AdditionalStatementProvider::with_default_optional(false);

        let mut type_comment = RustComment::new();
        let comment1 = "this is comment1";
        let comment2 = "this is comment2";
        type_comment.add_comment_line(comment1);
        type_comment.add_comment_line(comment2);
        let mut type_attr = RustAttribute::new();
        type_attr.add_attribute(RustAttributeKind::Derives(vec!["Clone", "Debug"]));
        additional_provider.set_all_type_comment(type_comment);
        additional_provider.set_all_type_attribute(type_attr);
        additional_provider.set_all_type_visibility(RustVisibility::Public);
        let generator = RustTypeStatementGenerator::new();
        let tobe = r#"// this is comment1
// this is comment2
#[derive(Clone,Debug)]
pub struct Test {
    id: usize,
}"#;
        let expect = generator.generate_case_composite(
            &type_name.into(),
            format!("    id: usize,\n"),
            &additional_provider,
        );
        assert_eq!(expect, tobe);
    }
    #[test]
    fn test_case_custum_with_all() {
        let type_name = "Test";
        let mut additional_provider = AdditionalStatementProvider::with_default_optional(false);
        let mut comment = RustComment::new();
        let comment1 = "this is comment1";
        let comment2 = "this is comment2";
        comment.add_comment_line(comment1);
        comment.add_comment_line(comment2);
        let mut attr = RustAttribute::new();
        attr.add_attribute(RustAttributeKind::Derives(vec!["Clone", "Debug"]));
        additional_provider.add_type_comment(type_name, comment);
        additional_provider.add_type_attribute(type_name, attr);
        additional_provider.add_type_visibility(type_name, RustVisibility::Public);
        let generator = RustTypeStatementGenerator::new();
        let tobe = r#"// this is comment1
// this is comment2
#[derive(Clone,Debug)]
pub struct Test {
    id: usize,
}"#;
        assert_eq!(
            generator.generate_case_composite(
                &type_name.into(),
                format!("    id: usize,\n"),
                &additional_provider
            ),
            tobe
        );
    }
    #[test]
    fn test_case_custum_with_comment() {
        let mut additional_provider = AdditionalStatementProvider::with_default_optional(false);
        let mut comment = RustComment::new();
        let comment1 = "this is comment1";
        let comment2 = "this is comment2";
        let type_name: TypeName = "Test".into();
        comment.add_comment_line(comment1);
        comment.add_comment_line(comment2);
        additional_provider.add_type_comment(type_name.clone(), comment);
        let generator = RustTypeStatementGenerator::new();
        let tobe = r#"// this is comment1
// this is comment2
struct Test {
    id: usize,
}"#;
        assert_eq!(
            generator.generate_case_composite(
                &type_name,
                format!("    id: usize,\n"),
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
                format!("    id: usize,\n"),
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
        additional_provider.add_type_comment(type_name.clone(), comment);
        let mapper = RustLangMapper;
        let primitive_type =
            AliasTypeStructure::new(type_name.clone(), make_primitive_type(make_string()));
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
        let primitive_type = AliasTypeStructure::new(type_name, make_primitive_type(make_string()));
        let generator = RustTypeStatementGenerator::new();
        let tobe = format!("type Test = String;");
        assert_eq!(
            generator.generate_case_primitive(&primitive_type, &mapper, &additional_provider),
            tobe
        );
    }
}
