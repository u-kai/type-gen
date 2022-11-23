use lang_common::{
    type_defines::{
        additional_defines::additional_statement::AdditionalStatementProvider,
        generators::generator::TypeDefineGenerator,
    },
    types::{property_key::PropertyKey, structures::TypeStructure, type_name::TypeName},
};

use super::{
    additional_statements::{RustComment, RustVisibility},
    attribute::RustAttribute,
    mapper::RustLangMapper,
    property_generator::RustPropertyStatementGenerator,
    type_statement_generator::RustTypeStatementGenerator,
};

pub struct RustTypeDefainGenerator {
    inner: TypeDefineGenerator<
        RustTypeStatementGenerator,
        RustPropertyStatementGenerator,
        RustLangMapper,
        AdditionalStatementProvider<RustVisibility, RustComment, RustAttribute>,
    >,
}
impl RustTypeDefainGenerator {
    pub fn new(
        mapper: RustLangMapper,
        property_generator: RustPropertyStatementGenerator,
        type_generator: RustTypeStatementGenerator,
        additional_provider: AdditionalStatementProvider<
            RustVisibility,
            RustComment,
            RustAttribute,
        >,
    ) -> Self {
        Self {
            inner: TypeDefineGenerator::new(
                type_generator,
                property_generator,
                mapper,
                additional_provider,
            ),
        }
    }
    pub fn generate_one(&self, type_structure: TypeStructure) -> String {
        self.inner.generate(type_structure)
    }
    pub fn generate(&self, type_structures: Vec<TypeStructure>) -> Vec<String> {
        type_structures
            .into_iter()
            .map(|types| self.inner.generate(types))
            .collect()
    }
}

pub struct RustTypeDefainGeneratorBuilder {
    inner: AdditionalStatementProvider<RustVisibility, RustComment, RustAttribute>,
}
impl<'a> RustTypeDefainGeneratorBuilder {
    pub fn new() -> Self {
        Self {
            inner: AdditionalStatementProvider::new(),
        }
    }
    pub fn build(self) -> RustTypeDefainGenerator {
        let mapper = RustLangMapper;
        let property_generator = RustPropertyStatementGenerator::new();
        let type_generator = RustTypeStatementGenerator::new();
        RustTypeDefainGenerator::new(mapper, property_generator, type_generator, self.inner)
    }
    pub fn add_type_attribute(
        mut self,
        type_name: impl Into<TypeName>,
        attribute: RustAttribute,
    ) -> Self {
        self.inner.add_type_attribute(type_name, attribute);
        self
    }
    pub fn add_property_attribute(
        mut self,
        type_name: impl Into<TypeName>,
        property_key: impl Into<PropertyKey>,
        attribute: RustAttribute,
    ) -> Self {
        self.inner
            .add_property_attribute(type_name, property_key, attribute);
        self
    }
    pub fn add_type_comment(
        mut self,
        type_name: impl Into<TypeName>,
        comment: RustComment,
    ) -> Self {
        self.inner.add_type_comment(type_name, comment);
        self
    }
    pub fn add_property_comment(
        mut self,
        type_name: impl Into<TypeName>,
        property_key: impl Into<PropertyKey>,
        comment: RustComment,
    ) -> Self {
        self.inner
            .add_property_comment(type_name, property_key, comment);
        self
    }
    pub fn add_optional(
        mut self,
        type_name: impl Into<TypeName>,
        property_key: impl Into<PropertyKey>,
    ) -> Self {
        self.inner.add_optional(type_name, property_key);
        self
    }
    pub fn add_require(
        mut self,
        type_name: impl Into<TypeName>,
        property_key: impl Into<PropertyKey>,
    ) -> Self {
        self.inner.add_require(type_name, property_key);
        self
    }
    pub fn add_type_visibility(
        mut self,
        type_name: impl Into<TypeName>,
        visibility: RustVisibility,
    ) -> Self {
        self.inner.add_type_visibility(type_name, visibility);
        self
    }
    pub fn add_property_visibility(
        mut self,
        type_name: impl Into<TypeName>,
        property_key: impl Into<PropertyKey>,
        visibility: RustVisibility,
    ) -> Self {
        self.inner
            .add_property_visibility(type_name, property_key, visibility);
        self
    }
}

#[cfg(test)]
mod test_type_define_generator {
    use lang_common::types::{
        primitive_type::primitive_type_factories::{make_string, make_usize},
        property_type::property_type_factories::{
            make_array_type, make_custom_type, make_primitive_type,
        },
        structures::TypeStructure,
    };

    use super::RustTypeDefainGeneratorBuilder;

    #[test]
    fn integration_test() {
        let root = TypeStructure::make_composite(
            "Root",
            vec![
                ("id", make_primitive_type(make_usize())),
                ("data", make_array_type(make_custom_type("RootData"))),
            ],
        );
        let root_data = TypeStructure::make_composite(
            "RootData",
            vec![(
                "results",
                make_array_type(make_custom_type("RootDataResults")),
            )],
        );
        let root_results = TypeStructure::make_composite(
            "RootDataResults",
            vec![
                ("name", make_primitive_type(make_string())),
                ("age", make_primitive_type(make_usize())),
                ("accountId", make_primitive_type(make_string())),
            ],
        );
        let builder = RustTypeDefainGeneratorBuilder::new();
        //    .add_require(root, )
    }
}
