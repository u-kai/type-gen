use lang_common::type_defines::{
    additional_defines::additional_statement::{AdditionalStatement, AdditionalStatementProvider},
    generators::generator::TypeStatementGenerator,
};

use super::{
    additional_statements::{RustComment, RustVisibility},
    mapper::RustLangMapper,
};

pub struct RustTypeStatementGenerator {}
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
        String::new()
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
            "{}{} {} {{{}}}",
            result,
            Self::TYPE_PREFIX,
            type_name.as_str(),
            properties_statement
        )
    }
}
