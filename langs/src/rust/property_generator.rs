use lang_common::{
    type_defines::{
        additional_defines::additional_statement::{
            AdditionalStatement, AdditionalStatementProvider,
        },
        generators::{generator::PropertyStatementGenerator, mapper::LangTypeMapper},
    },
    types::{property_key::PropertyKey, type_name::TypeName},
};
use npc::{convertor::NamingPrincipalConvertor, naming_principal::NamingPrincipal};

use super::{
    additional_statements::{RustComment, RustVisibility},
    attribute::RustAttribute,
    mapper::RustLangMapper,
    reserved_words::RustReservedWords,
};
struct RustPropertyKey<'a> {
    convertor: NamingPrincipalConvertor<'a>,
}

impl<'a> RustPropertyKey<'a> {
    fn new(original: &'a PropertyKey) -> Self {
        Self {
            convertor: NamingPrincipalConvertor::new(original.as_str()),
        }
    }
    fn property_str(&self, reserved_words: &RustReservedWords, visibility: &str) -> String {
        format!(
            "{rename_attr}{head}{visibility}{property_key}",
            head = RUST_PROPERTY_HEAD_SPACE,
            rename_attr = self.rename_attr(),
            visibility = visibility,
            property_key = self.convert_key(reserved_words)
        )
    }
    fn convert_key(&self, reserved_words: &RustReservedWords) -> String {
        let converted = Self::replace_cannot_use_char(&self.convertor.to_snake());
        if reserved_words.is_reserved_keywords(&converted) {
            return Self::from_reserved_word(&converted);
        }
        if reserved_words.is_strict_keywords(&converted) {
            return Self::from_strict_word(&converted);
        }
        converted
    }
    fn from_strict_word(strict_words: &str) -> String {
        format!("{}_", strict_words)
    }
    fn from_reserved_word(reserved_words: &str) -> String {
        format!(r"r#{}", reserved_words)
    }
    fn rename_attr(&self) -> String {
        if Self::containe_cannot_use_char(self.convertor.original()) || !self.convertor.is_snake() {
            format!(
                "{head}#[serde(rename = \"{original}\")]\n",
                head = RUST_PROPERTY_HEAD_SPACE,
                original = self.convertor.original()
            )
        } else {
            "".to_string()
        }
    }
    fn replace_cannot_use_char(str: &str) -> String {
        str.replace(Self::cannot_use_char, "")
    }
    fn containe_cannot_use_char(str: &str) -> bool {
        str.contains(Self::cannot_use_char)
    }
    fn cannot_use_char(c: char) -> bool {
        match c {
            ':' | ';' | '#' | '$' | '%' | '&' | '~' | '=' | '|' | '\"' | '\'' | '{' | '}' | '?'
            | '!' | '<' | '>' | '[' | ']' | '*' | '^' => true,
            _ => false,
        }
    }
}
pub struct RustPropertyStatementGenerator {}
pub const RUST_PROPERTY_HEAD_SPACE: &'static str = "    ";
impl RustPropertyStatementGenerator {
    const NEXT_LINE: &'static str = ",\n";
    pub fn new() -> Self {
        Self {}
    }
    fn make_additional(
        &self,
        type_name: &TypeName,
        property_key: &PropertyKey,
        additional_provider: &AdditionalStatementProvider<
            RustVisibility,
            RustComment,
            RustAttribute,
        >,
    ) -> String {
        let mut additional = String::new();
        if let Some(comment) = additional_provider.get_property_comment(type_name, property_key) {
            additional += &comment;
        };
        if let Some(attribute) = additional_provider.get_property_attribute(type_name, property_key)
        {
            additional += &attribute;
        };
        additional
    }
}
impl<'a>
    PropertyStatementGenerator<
        RustLangMapper,
        AdditionalStatementProvider<RustVisibility, RustComment, RustAttribute>,
    > for RustPropertyStatementGenerator
{
    fn generate(
        &self,
        type_name: &lang_common::types::type_name::TypeName,
        property_key: &lang_common::types::property_key::PropertyKey,
        property_type: &lang_common::types::property_type::PropertyType,
        mapper: &RustLangMapper,
        additional_provider: &AdditionalStatementProvider<
            RustVisibility,
            RustComment,
            RustAttribute,
        >,
    ) -> String {
        let additional = self.make_additional(type_name, property_key, additional_provider);
        let reserved_words = RustReservedWords::new();
        let property_str = RustPropertyKey::new(property_key).property_str(
            &reserved_words,
            additional_provider.get_property_visibility(type_name, property_key),
        );
        let property_type = if additional_provider.is_property_optional(type_name, &property_key) {
            mapper.case_optional_type(mapper.case_property_type(property_type))
        } else {
            mapper.case_property_type(property_type)
        };
        format!(
            "{additional}{property_key}: {property_type}{next_line}",
            additional = additional,
            property_key = property_str,
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
        additional_provider.add_property_attribute(type_name.clone(), property_key.clone(), attr);
        additional_provider.add_property_comment(type_name.clone(), property_key.clone(), comment);
        additional_provider.add_optional(type_name.clone(), property_key.clone());
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
        additional_provider.add_property_comment(type_name.clone(), property_key.clone(), comment);
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
    fn test_case_not_use_str_and_not_snake() {
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id:Value".into();
        let property_type = make_primitive_type(make_usize());
        let generator = RustPropertyStatementGenerator::new();
        let mapper = RustLangMapper;
        let additional_provider = AdditionalStatementProvider::with_default_optional(false);
        let tobe = format!(
            "{head}#[serde(rename = \"id:Value\")]\n{head}id_value: usize,\n",
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
    fn test_case_not_use_str() {
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id:value".into();
        let property_type = make_primitive_type(make_usize());
        let generator = RustPropertyStatementGenerator::new();
        let mapper = RustLangMapper;
        let additional_provider = AdditionalStatementProvider::with_default_optional(false);
        let tobe = format!(
            "{head}#[serde(rename = \"id:value\")]\n{head}idvalue: usize,\n",
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
