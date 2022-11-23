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
        format!(
            "type {name} = {type_str};",
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
            "{}{} {} {{{}}}",
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

    use crate::rust::mapper::RustLangMapper;

    use super::RustTypeStatementGenerator;

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
