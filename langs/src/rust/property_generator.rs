use lang_common::type_defines::{
    additional_defines::additional_statement::{AdditionalStatement, AdditionalStatementProvider},
    generators::{generator::PropertyStatementGenerator, mapper::LangTypeMapper},
};
use npc::{convertor::NamingPrincipalConvertor, naming_principal::NamingPrincipal};

use super::{
    additional_statements::{RustComment, RustVisibility},
    attribute::RustAttribute,
    mapper::RustLangMapper,
    reserved_words::RustReservedWords,
};
struct RustPropertyStatementGenerator {
    reserved_words: RustReservedWords,
}
pub const RUST_PROPERTY_HEAD_SPACE: &'static str = "    ";
impl RustPropertyStatementGenerator {
    const NEXT_LINE: &'static str = ",\n";
    pub fn new() -> Self {
        Self {
            reserved_words: RustReservedWords::new(),
        }
    }
}
impl<'a>
    PropertyStatementGenerator<
        RustLangMapper,
        AdditionalStatementProvider<'a, RustVisibility, RustComment<'a>, RustAttribute>,
    > for RustPropertyStatementGenerator
{
    fn generate(
        &self,
        type_name: &lang_common::types::type_name::TypeName,
        property_key: &lang_common::types::property_key::PropertyKey,
        property_type: &lang_common::types::property_type::PropertyType,
        mapper: &RustLangMapper,
        additional_statement: &AdditionalStatementProvider<
            'a,
            RustVisibility,
            RustComment<'a>,
            RustAttribute,
        >,
    ) -> String {
        let mut additional = String::new();
        if let Some(comment) = additional_statement.get_property_comment(type_name, property_key) {
            additional += &comment;
        };
        if let Some(attribute) =
            additional_statement.get_property_attribute(type_name, property_key)
        {
            additional += &attribute;
        };
        additional += additional_statement.get_property_visibility(type_name, property_key);
        if !NamingPrincipal::is_snake(property_key.as_str()) {
            additional += &format!(
                "{head}#[serde(rename = \"{original}\")]\n",
                head = RUST_PROPERTY_HEAD_SPACE,
                original = property_key.as_str()
            )
        }
        let property_key_str = NamingPrincipalConvertor::new(property_key.as_str()).to_snake();
        let property_key_str = self.reserved_words.get_or_origin(&property_key_str);
        let property_type = if additional_statement.is_property_optional(type_name, property_key) {
            mapper.case_optional_type(mapper.case_property_type(property_type))
        } else {
            mapper.case_property_type(property_type)
        };
        format!(
            "{additional}{head}{property_key}: {property_type}{next_line}",
            additional = additional,
            head = RUST_PROPERTY_HEAD_SPACE,
            property_key = property_key_str,
            property_type = property_type,
            next_line = Self::NEXT_LINE
        )
    }
}

#[cfg(test)]
mod test_rust_property_geneartor {
    use lang_common::{
        type_defines::{
            additional_defines::additional_statement::AdditionalStatementProvider,
            generators::generator::PropertyStatementGenerator,
        },
        types::{
            primitive_type::primitive_type_factories::{make_string, make_usize},
            property_key::PropertyKey,
            property_type::property_type_factories::{
                make_array_type, make_custom_type, make_primitive_type,
            },
            type_name::TypeName,
        },
    };

    use crate::rust::{
        additional_statements::RustComment,
        attribute::{RustAttribute, RustAttributeKind},
        mapper::RustLangMapper,
        property_generator::RUST_PROPERTY_HEAD_SPACE,
    };

    use super::RustPropertyStatementGenerator;

    #[test]
    fn test_case_primitive_all() {
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "accountId".into();
        let property_type = make_primitive_type(make_string());
        let generator = RustPropertyStatementGenerator::new();
        let mapper = RustLangMapper;
        let mut additional_provider = AdditionalStatementProvider::with_default_optional(false);
        let mut attr = RustAttribute::new();
        attr.add_attribute(RustAttributeKind::Test);
        let mut comment = RustComment::new();
        comment.add_comment_line("this is test");
        additional_provider.add_property_attribute(&type_name, &property_key, attr);
        additional_provider.add_property_comment(&type_name, &property_key, comment);
        additional_provider.add_optional(&type_name, &property_key);
        let tobe = format!(
            "{head}// this is test\n{head}#[test]\n{head}#[serde(rename = \"accountId\")]\n{head}account_id: Option<String>,\n",
            head = RUST_PROPERTY_HEAD_SPACE,
        );
        assert_eq!(
            generator.generate(
                &type_name,
                &property_key,
                &property_type,
                &mapper,
                &additional_provider
            ),
            tobe
        );
    }
    #[test]
    fn test_case_primitive_and_use_camel_case_property_key() {
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "accountId".into();
        let property_type = make_primitive_type(make_string());
        let generator = RustPropertyStatementGenerator::new();
        let mapper = RustLangMapper;
        let additional_provider = AdditionalStatementProvider::with_default_optional(false);
        let tobe = format!(
            "{head}#[serde(rename = \"accountId\")]\n{head}account_id: String,\n",
            head = RUST_PROPERTY_HEAD_SPACE,
        );
        assert_eq!(
            generator.generate(
                &type_name,
                &property_key,
                &property_type,
                &mapper,
                &additional_provider
            ),
            tobe
        );
    }
    #[test]
    fn test_case_primitive_and_use_reserved_words() {
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "type".into();
        let property_type = make_primitive_type(make_string());
        let generator = RustPropertyStatementGenerator::new();
        let mapper = RustLangMapper;
        let additional_provider = AdditionalStatementProvider::with_default_optional(false);
        let tobe = format!("{head}r#type: String,\n", head = RUST_PROPERTY_HEAD_SPACE,);
        assert_eq!(
            generator.generate(
                &type_name,
                &property_key,
                &property_type,
                &mapper,
                &additional_provider
            ),
            tobe
        );
    }
    #[test]
    fn test_case_custom_array_option_with_comment() {
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let mut comment = RustComment::new();
        let comment1 = "this is comment1";
        let comment2 = "this is comment2";
        comment.add_comment_line(comment1);
        comment.add_comment_line(comment2);
        let property_type = make_array_type(make_custom_type("TestId"));
        let generator = RustPropertyStatementGenerator::new();
        let mapper = RustLangMapper;
        let mut additional_provider = AdditionalStatementProvider::with_default_optional(true);
        additional_provider.add_property_comment(&type_name, &property_key, comment);
        let tobe = format!(
            "{head}// {comment1}\n{head}// {comment2}\n{head}id: Option<Vec<TestId>>,\n",
            head = RUST_PROPERTY_HEAD_SPACE,
            comment1 = comment1,
            comment2 = comment2
        );
        assert_eq!(
            generator.generate(
                &type_name,
                &property_key,
                &property_type,
                &mapper,
                &additional_provider
            ),
            tobe
        );
    }
    #[test]
    fn test_case_custom_array_option() {
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type = make_array_type(make_custom_type("TestId"));
        let generator = RustPropertyStatementGenerator::new();
        let mapper = RustLangMapper;
        let additional_provider = AdditionalStatementProvider::with_default_optional(true);
        let tobe = "    id: Option<Vec<TestId>>,\n".to_string();
        assert_eq!(
            generator.generate(
                &type_name,
                &property_key,
                &property_type,
                &mapper,
                &additional_provider
            ),
            tobe
        );
    }
    #[test]
    fn test_case_custom_all_none_additional() {
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type = make_custom_type("TestId");
        let generator = RustPropertyStatementGenerator::new();
        let mapper = RustLangMapper;
        let additional_provider = AdditionalStatementProvider::with_default_optional(false);
        let tobe = "    id: TestId,\n".to_string();
        assert_eq!(
            generator.generate(
                &type_name,
                &property_key,
                &property_type,
                &mapper,
                &additional_provider
            ),
            tobe
        );
    }
    #[test]
    fn test_case_primitive_all_none_additional() {
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type = make_primitive_type(make_usize());
        let generator = RustPropertyStatementGenerator::new();
        let mapper = RustLangMapper;
        let additional_provider = AdditionalStatementProvider::with_default_optional(false);
        let tobe = "    id: usize,\n".to_string();
        assert_eq!(
            generator.generate(
                &type_name,
                &property_key,
                &property_type,
                &mapper,
                &additional_provider
            ),
            tobe
        );
    }
}
