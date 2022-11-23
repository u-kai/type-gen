use lang_common::type_defines::{
    additional_defines::additional_statement::{AdditionalStatement, AdditionalStatementProvider},
    generators::{generator::PropertyStatementGenerator, mapper::LangTypeMapper},
};

use super::{
    additional_statements::{RustComment, RustVisibility},
    mapper::RustLangMapper,
    reserved_words::RustReservedWords,
};
struct RustPropertyStatementGenerator {
    reserved_words: RustReservedWords,
}
impl RustPropertyStatementGenerator {
    const HEAD_SPACE: &'static str = "    ";
    const NEXT_LINE: &'static str = ",\n";
    fn new() -> Self {
        Self {
            reserved_words: RustReservedWords::new(),
        }
    }
}
impl<'a>
    PropertyStatementGenerator<
        RustLangMapper,
        AdditionalStatementProvider<'a, RustVisibility, RustComment<'a>>,
    > for RustPropertyStatementGenerator
{
    fn generate(
        &self,
        type_name: &lang_common::types::type_name::TypeName,
        property_key: &lang_common::types::property_key::PropertyKey,
        property_type: &lang_common::types::property_type::PropertyType,
        mapper: &RustLangMapper,
        additional_statement: &AdditionalStatementProvider<'a, RustVisibility, RustComment<'a>>,
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
        let property_type = if additional_statement.is_property_optional(type_name, property_key) {
            mapper.case_optional_type(mapper.case_property_type(property_type))
        } else {
            mapper.case_property_type(property_type)
        };
        format!(
            "{head}{additional}{property_key}: {property_type}{next_line}",
            head = Self::HEAD_SPACE,
            additional = additional,
            property_key = property_key.as_str(),
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
            primitive_type::primitive_type_factories::make_usize,
            property_key::PropertyKey,
            property_type::{property_type_factories::make_primitive_type, PropertyType},
            type_name::TypeName,
        },
    };

    use crate::rust::mapper::RustLangMapper;

    use super::RustPropertyStatementGenerator;

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
