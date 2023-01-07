use lang_common::{
    type_defines::{
        additional_defines::additional_statement::{
            AdditionalStatement, AdditionalStatementProvider,
        },
        generators::{mapper::LangTypeMapper, type_define_generator::TypeStatementGenerator},
    },
    types::type_name::TypeName,
};

use super::{
    additional_statements::{RustComment, RustVisibility},
    attribute::RustAttribute,
    mapper::RustLangMapper,
    reserved_words::replace_cannot_use_char,
};

pub struct RustTypeStatementGenerator {
    additional_provider: AdditionalStatementProvider<RustVisibility, RustComment, RustAttribute>,
}
impl<'a> RustTypeStatementGenerator {
    pub fn new(
        additional_provider: AdditionalStatementProvider<
            RustVisibility,
            RustComment,
            RustAttribute,
        >,
    ) -> Self {
        Self {
            additional_provider,
        }
    }
    fn make_additional_case_alias(&self, type_name: &TypeName) -> String {
        let mut result = String::new();
        if let Some(comment) = self.additional_provider.get_type_comment(type_name) {
            result += &comment;
        };
        result += self.additional_provider.get_type_visibility(type_name);
        result
    }
    fn make_additional_case_composite(&self, type_name: &TypeName) -> String {
        let mut result = String::new();
        if let Some(comment) = self.additional_provider.get_type_comment(type_name) {
            result += &comment;
        };
        if let Some(attribute) = self.additional_provider.get_type_attribute(type_name) {
            result += &attribute;
        };
        result += self.additional_provider.get_type_visibility(type_name);
        result
    }
}
impl<'a> TypeStatementGenerator<RustLangMapper> for RustTypeStatementGenerator {
    const TYPE_PREFIX: &'static str = "struct";
    fn generate_case_alias(
        &self,
        primitive_type: &lang_common::types::structures::AliasTypeStructure,
        mapper: &RustLangMapper,
        //        additional_statement: &AdditionalStatementProvider<
        //            RustVisibility,
        //            RustComment,
        //            RustAttribute,
        //        >,
    ) -> String {
        let additional = self.make_additional_case_alias(&primitive_type.name);
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
        //  additional_statement: &AdditionalStatementProvider<
        //      RustVisibility,
        //      RustComment,
        //      RustAttribute,
        //  >,
    ) -> String {
        let additional = self.make_additional_case_composite(type_name);
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
            generators::type_define_generator::TypeStatementGenerator,
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
        let generator = RustTypeStatementGenerator::new(additional_provider);
        let tobe = r#"// this is comment1
// this is comment2
#[derive(Clone,Debug)]
pub struct Test {
    id: usize,
}"#;
        let expect =
            generator.generate_case_composite(&type_name.into(), format!("    id: usize,\n"));
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
        let generator = RustTypeStatementGenerator::new(additional_provider);
        let tobe = r#"// this is comment1
// this is comment2
#[derive(Clone,Debug)]
pub struct Test {
    id: usize,
}"#;
        assert_eq!(
            generator.generate_case_composite(&type_name.into(), format!("    id: usize,\n"),),
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
        let generator = RustTypeStatementGenerator::new(additional_provider);
        let tobe = r#"// this is comment1
// this is comment2
struct Test {
    id: usize,
}"#;
        assert_eq!(
            generator.generate_case_composite(&type_name, format!("    id: usize,\n"),),
            tobe
        );
    }
    #[test]
    fn test_case_custum_all_none_additional() {
        let additional_provider = AdditionalStatementProvider::with_default_optional(false);
        let type_name: TypeName = "Test".into();
        let generator = RustTypeStatementGenerator::new(additional_provider);
        let tobe = r#"struct Test {
    id: usize,
}"#;
        assert_eq!(
            generator.generate_case_composite(&type_name, format!("    id: usize,\n"),),
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
        let generator = RustTypeStatementGenerator::new(additional_provider);
        let tobe = format!("// {comment1}\n// {comment2}\ntype Test = String;");
        assert_eq!(
            generator.generate_case_alias(&primitive_type, &mapper,),
            tobe
        );
    }
    #[test]
    fn test_case_alias_all_none_additional() {
        let additional_provider = AdditionalStatementProvider::with_default_optional(false);
        let type_name: TypeName = "Test".into();
        let mapper = RustLangMapper;
        let primitive_type = AliasTypeStructure::new(type_name, make_primitive_type(make_string()));
        let generator = RustTypeStatementGenerator::new(additional_provider);
        let tobe = format!("type Test = String;");
        assert_eq!(
            generator.generate_case_alias(&primitive_type, &mapper,),
            tobe
        );
    }
}
