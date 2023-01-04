use lang_common::{
    type_defines::{
        additional_defines::additional_statement::AdditionalStatementProvider,
        builder::TypeDefineBuilder, generators::generator::TypeDefineGenerator,
    },
    types::{property_key::PropertyKey, type_name::TypeName},
};

use super::{
    additional_statements::{RustComment, RustVisibility},
    attribute::RustAttribute,
    mapper::RustLangMapper,
    property_generator::RustPropertyStatementGenerator,
    type_statement_generator::RustTypeStatementGenerator,
};

pub struct RustTypeDefainGeneratorBuilder {
    inner: AdditionalStatementProvider<RustVisibility, RustComment, RustAttribute>,
}
impl RustTypeDefainGeneratorBuilder {
    pub fn new() -> Self {
        Self {
            inner: AdditionalStatementProvider::new(),
        }
    }
}
impl
    TypeDefineBuilder<
        RustTypeStatementGenerator,
        RustPropertyStatementGenerator,
        RustLangMapper,
        AdditionalStatementProvider<RustVisibility, RustComment, RustAttribute>,
        RustVisibility,
        RustComment,
        RustAttribute,
    > for RustTypeDefainGeneratorBuilder
{
    fn build(
        self,
    ) -> TypeDefineGenerator<
        RustTypeStatementGenerator,
        RustPropertyStatementGenerator,
        RustLangMapper,
        AdditionalStatementProvider<RustVisibility, RustComment, RustAttribute>,
    > {
        let mapper = RustLangMapper;
        let property_generator = RustPropertyStatementGenerator::new(self.inner.clone());
        let type_generator = RustTypeStatementGenerator::new(self.inner.clone());
        TypeDefineGenerator::new(type_generator, property_generator, mapper, self.inner)
    }
    fn set_all_type_optional(mut self, is_all_optioal: bool) -> Self {
        self.inner.set_all_type_optional(is_all_optioal);
        self
    }
    fn set_all_type_visibility(mut self, visibility: RustVisibility) -> Self {
        self.inner.set_all_type_visibility(visibility);
        self
    }
    fn set_all_property_visibility(mut self, visibility: RustVisibility) -> Self {
        self.inner.set_all_property_visibility(visibility);
        self
    }
    fn set_all_type_comment(mut self, comment: RustComment) -> Self {
        self.inner.set_all_type_comment(comment);
        self
    }
    fn set_all_property_comment(mut self, comment: RustComment) -> Self {
        self.inner.set_all_property_comment(comment);
        self
    }
    fn set_all_type_attribute(mut self, attribute: RustAttribute) -> Self {
        self.inner.set_all_type_attribute(attribute);
        self
    }
    fn set_all_property_attribute(mut self, attribute: RustAttribute) -> Self {
        self.inner.set_all_property_attribute(attribute);
        self
    }
    fn add_type_attribute(
        mut self,
        type_name: impl Into<TypeName>,
        attribute: RustAttribute,
    ) -> Self {
        self.inner.add_type_attribute(type_name, attribute);
        self
    }
    fn add_property_attribute(
        mut self,
        type_name: impl Into<TypeName>,
        property_key: impl Into<PropertyKey>,
        attribute: RustAttribute,
    ) -> Self {
        self.inner
            .add_property_attribute(type_name, property_key, attribute);
        self
    }
    fn add_type_comment(mut self, type_name: impl Into<TypeName>, comment: RustComment) -> Self {
        self.inner.add_type_comment(type_name, comment);
        self
    }
    fn add_property_comment(
        mut self,
        type_name: impl Into<TypeName>,
        property_key: impl Into<PropertyKey>,
        comment: RustComment,
    ) -> Self {
        self.inner
            .add_property_comment(type_name, property_key, comment);
        self
    }
    fn add_optional(
        mut self,
        type_name: impl Into<TypeName>,
        property_key: impl Into<PropertyKey>,
    ) -> Self {
        self.inner.add_optional(type_name, property_key);
        self
    }
    fn add_require(
        mut self,
        type_name: impl Into<TypeName>,
        property_key: impl Into<PropertyKey>,
    ) -> Self {
        self.inner.add_require(type_name, property_key);
        self
    }
    fn add_type_visibility(
        mut self,
        type_name: impl Into<TypeName>,
        visibility: RustVisibility,
    ) -> Self {
        self.inner.add_type_visibility(type_name, visibility);
        self
    }
    fn add_property_visibility(
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
    use lang_common::{
        type_defines::builder::TypeDefineBuilder,
        types::{
            primitive_type::primitive_type_factories::{make_string, make_usize},
            property_type::property_type_factories::{
                make_array_type, make_custom_type, make_primitive_type,
            },
            structures::TypeStructure,
        },
    };

    use crate::rust::{
        additional_statements::{RustComment, RustVisibility},
        attribute::{RustAttribute, RustAttributeKind},
    };

    use super::RustTypeDefainGeneratorBuilder;
    #[test]
    fn integration_test_case_set_all() {
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
        let root_data_results = TypeStructure::make_composite(
            "RootDataResults",
            vec![
                ("name", make_primitive_type(make_string())),
                ("age", make_primitive_type(make_usize())),
                ("accountId", make_primitive_type(make_string())),
            ],
        );
        let type_comment = RustComment::from("this is type");
        let type_attribute = RustAttribute::from_derives(vec!["Clone", "Debug"]);
        let type_visibility = RustVisibility::Public;
        let property_comment = RustComment::from("this is property");
        let property_attribute =
            RustAttribute::from(RustAttributeKind::Original("allow(unuse)".to_string()));
        let property_visibility = RustVisibility::Public;
        let is_all_optional = true;
        let generator = RustTypeDefainGeneratorBuilder::new()
            .set_all_type_attribute(type_attribute)
            .set_all_type_comment(type_comment)
            .set_all_type_visibility(type_visibility)
            .set_all_property_attribute(property_attribute)
            .set_all_property_comment(property_comment)
            .set_all_property_visibility(property_visibility)
            .set_all_type_optional(is_all_optional)
            .build();
        let tobe = vec![
            r#"// this is type
#[derive(Clone,Debug)]
pub struct Root {
    // this is property
    #[allow(unuse)]
    pub data: Option<Vec<RootData>>,
    // this is property
    #[allow(unuse)]
    pub id: Option<usize>,
}"#,
            r#"// this is type
#[derive(Clone,Debug)]
pub struct RootData {
    // this is property
    #[allow(unuse)]
    pub results: Option<Vec<RootDataResults>>,
}"#,
            r#"// this is type
#[derive(Clone,Debug)]
pub struct RootDataResults {
    // this is property
    #[allow(unuse)]
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    // this is property
    #[allow(unuse)]
    pub age: Option<usize>,
    // this is property
    #[allow(unuse)]
    pub name: Option<String>,
}"#,
        ];
        let expect = generator.generate(vec![root, root_data, root_data_results]);
        for e in &expect {
            println!("{}", e);
        }
        assert_eq!(expect, tobe)
    }

    #[test]
    fn integration_test_case_cannot_use() {
        let root = TypeStructure::make_composite(
            "Root",
            vec![
                ("id", make_primitive_type(make_usize())),
                (
                    "data:valu:e",
                    make_array_type(make_custom_type("RootDatavalue")),
                ),
            ],
        );
        let root_data = TypeStructure::make_composite(
            "RootDatavalue",
            vec![(
                "results",
                make_array_type(make_custom_type("RootDatavalueResults")),
            )],
        );
        let root_data_results = TypeStructure::make_composite(
            "RootDatavalueResults",
            vec![
                ("name", make_primitive_type(make_string())),
                ("age", make_primitive_type(make_usize())),
                ("accountId", make_primitive_type(make_string())),
            ],
        );
        let generator = RustTypeDefainGeneratorBuilder::new()
            .add_require("Root", "id")
            .add_type_visibility("Root", RustVisibility::Public)
            .add_property_visibility("Root", "id", RustVisibility::Public)
            .set_all_type_attribute(RustAttribute::from_derives(vec!["Clone", "Debug"]))
            .build();
        let tobe = vec![
            r#"#[derive(Clone,Debug)]
pub struct Root {
    #[serde(rename = "data:valu:e")]
    datavalue: Option<Vec<RootDatavalue>>,
    pub id: usize,
}"#,
            r#"#[derive(Clone,Debug)]
struct RootDatavalue {
    results: Option<Vec<RootDatavalueResults>>,
}"#,
            r#"#[derive(Clone,Debug)]
struct RootDatavalueResults {
    #[serde(rename = "accountId")]
    account_id: Option<String>,
    age: Option<usize>,
    name: Option<String>,
}"#,
        ];
        assert_eq!(
            generator.generate(vec![root, root_data, root_data_results]),
            tobe
        )
    }
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
        let root_data_results = TypeStructure::make_composite(
            "RootDataResults",
            vec![
                ("name", make_primitive_type(make_string())),
                ("age", make_primitive_type(make_usize())),
                ("accountId", make_primitive_type(make_string())),
            ],
        );
        let id_comment = "id is must set";
        let root_data_results_comment = "data results";
        let generator = RustTypeDefainGeneratorBuilder::new()
            .add_require("Root", "id")
            .add_property_comment("Root", "id", RustComment::from(id_comment))
            .add_type_comment(
                "RootDataResults",
                RustComment::from(root_data_results_comment),
            )
            .add_type_visibility("Root", RustVisibility::Public)
            .add_property_visibility("Root", "id", RustVisibility::Public)
            .set_all_type_attribute(RustAttribute::from_derives(vec!["Clone", "Debug"]))
            .build();
        let tobe = vec![
            r#"#[derive(Clone,Debug)]
pub struct Root {
    data: Option<Vec<RootData>>,
    // id is must set
    pub id: usize,
}"#,
            r#"#[derive(Clone,Debug)]
struct RootData {
    results: Option<Vec<RootDataResults>>,
}"#,
            r#"// data results
#[derive(Clone,Debug)]
struct RootDataResults {
    #[serde(rename = "accountId")]
    account_id: Option<String>,
    age: Option<usize>,
    name: Option<String>,
}"#,
        ];
        assert_eq!(
            generator.generate(vec![root, root_data, root_data_results]),
            tobe
        )
    }
}
