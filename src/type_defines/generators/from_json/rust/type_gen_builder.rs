use crate::type_defines::{
    generators::from_json::{
        lang_common::{
            field_statements::field_comment::BaseFieldComment,
            optional_checker::BaseOptionalChecker, type_statements::type_comment::BaseTypeComment,
        },
        type_define_generator::TypeDefineGenerator,
    },
    statement_parts::{field_key::Fieldkey, type_key::TypeKey},
};

use super::{
    field_statements::{
        field_attr::{RustFieldAttribute, RustFieldAttributeStore},
        field_statement::RustfieldStatement,
        field_visibility::RustFieldVisibilityProvider,
        reserved_words::RustReservedWords,
    },
    json_lang_mapper::JsonRustMapper,
    off_side_rule::RustOffSideRule,
    rust_visibility::RustVisibility,
    type_statements::{
        type_attr::{RustTypeAttribute, RustTypeAttributeStore},
        type_statement::RustTypeStatement,
        type_visibility::RustTypeVisibilityProvider,
    },
};

struct TypeStatements {
    comment: BaseTypeComment,
    visi: RustTypeVisibilityProvider,
    attr: RustTypeAttributeStore,
}
struct FieldStatements<'a> {
    comment: BaseFieldComment,
    visi: RustFieldVisibilityProvider,
    attr: RustFieldAttributeStore<'a>,
}
pub struct RustTypeGeneratorBuilder<'a> {
    type_statements: TypeStatements,
    field_statements: FieldStatements<'a>,
    opsional_checker: BaseOptionalChecker,
}

impl<'a> RustTypeGeneratorBuilder<'a> {
    pub fn new() -> Self {
        let t = TypeStatements {
            comment: BaseTypeComment::new("//"),
            visi: RustTypeVisibilityProvider::new(),
            attr: RustTypeAttributeStore::new(),
        };
        let f = FieldStatements {
            comment: BaseFieldComment::new("//"),
            visi: RustFieldVisibilityProvider::new(),
            attr: RustFieldAttributeStore::new(),
        };
        Self {
            type_statements: t,
            field_statements: f,
            opsional_checker: BaseOptionalChecker::default(),
        }
    }
    // build
    pub fn build(
        self,
        root_struct_name: &str,
    ) -> TypeDefineGenerator<
        JsonRustMapper,
        RustTypeStatement,
        RustfieldStatement<'a>,
        RustOffSideRule,
    > {
        let mapper = JsonRustMapper::new();
        let type_statement = RustTypeStatement::new(
            self.type_statements.comment,
            self.type_statements.visi,
            self.type_statements.attr,
        );
        let field_statement = RustfieldStatement::new(
            self.field_statements.comment,
            self.field_statements.attr,
            self.field_statements.visi,
            RustReservedWords::new(),
        );
        TypeDefineGenerator::new(
            root_struct_name,
            mapper,
            type_statement,
            field_statement,
            RustOffSideRule::new(),
            self.opsional_checker,
        )
    }
    // visi
    pub fn set_visibility_to_all_struct(mut self, visi: RustVisibility) -> Self {
        self.type_statements.visi.set_all_visibility(visi);
        self
    }
    pub fn set_visibility_to_all_field(mut self, visi: RustVisibility) -> Self {
        self.field_statements.visi.set_all_visibility(visi);
        self
    }
    pub fn set_visibility_to_struct(mut self, struct_name: &str, visi: RustVisibility) -> Self {
        self.type_statements.visi.add_visibility(struct_name, visi);
        self
    }
    pub fn set_visibility_to_field(mut self, field_key: &str, visi: RustVisibility) -> Self {
        self.field_statements.visi.add_visibility(field_key, visi);
        self
    }
    // attr
    pub fn set_attr_to_all_struct(mut self, attrs: Vec<RustTypeAttribute>) -> Self {
        self.type_statements.attr.set_attr_all(attrs);
        self
    }
    pub fn set_attr_to_all_field(mut self, attrs: Vec<RustFieldAttribute>) -> Self {
        self.field_statements.attr.set_attr_all(attrs);
        self
    }
    pub fn add_attr_to_struct(mut self, struct_name: &str, attr: RustTypeAttribute) -> Self {
        self.type_statements.attr.add_attr(struct_name, attr);
        self
    }
    pub fn add_attr_to_field(
        mut self,
        type_key: &'a TypeKey,
        field_key: &'a Fieldkey,
        attr: RustFieldAttribute,
    ) -> Self {
        self.field_statements
            .attr
            .add_attr(type_key, field_key, attr);
        self
    }
    // comment
    pub fn add_comment_to_struct(mut self, struct_name: &str, comment: &str) -> Self {
        self.type_statements
            .comment
            .add_comment(struct_name, comment);
        self
    }
    pub fn add_comment_to_field(mut self, field_key: &str, comment: &str) -> Self {
        self.field_statements
            .comment
            .add_comment(field_key, comment);
        self
    }
    // optional
    pub fn all_require(mut self) -> Self {
        self.opsional_checker = BaseOptionalChecker::new(false);
        self
    }
    pub fn all_optional(mut self) -> Self {
        self.opsional_checker = BaseOptionalChecker::new(true);
        self
    }
    pub fn add_require(mut self, struct_name: &'static str, field_key: &'static str) -> Self {
        self.opsional_checker.add_require(struct_name, field_key);
        self
    }
    pub fn add_optional(mut self, struct_name: &'static str, field_key: &'static str) -> Self {
        self.opsional_checker.add_optional(struct_name, field_key);
        self
    }
}

#[cfg(test)]
mod test_rust_type_gen_builder {
    use super::*;

    #[test]
    fn test_rust_builder() {
        let json = r#"
            {
                "data":[
                    {
                        "userId":12345,
                        "test":"test-string",
                        "entities":{
                            "id":0
                        }
                    }
                ]
            }
        "#;
        let tobe = r#"#[derive(Serialize,Desrialize)]
pub struct TestJson {
    data: Vec<TestJsonData>,
}

#[derive(Serialize,Desrialize)]
struct TestJsonData {
    entities: Option<TestJsonDataEntities>,
    test: Option<String>,
    #[serde(rename = "userId")]
    user_id: Option<i64>,
}

#[derive(Serialize,Desrialize)]
pub struct TestJsonDataEntities {
    id: Option<i64>,
}

"#
        .to_string();
        let rust = RustTypeGeneratorBuilder::new()
            .add_attr_to_struct(
                "TestJson",
                RustTypeAttribute::Derive(vec!["Serialize".to_string(), "Desrialize".to_string()]),
            )
            .add_require("TestJson", "data")
            .add_attr_to_struct(
                "TestJsonData",
                RustTypeAttribute::Derive(vec!["Serialize".to_string(), "Desrialize".to_string()]),
            )
            .add_attr_to_struct(
                "TestJsonDataEntities",
                RustTypeAttribute::Derive(vec!["Serialize".to_string(), "Desrialize".to_string()]),
            )
            .set_visibility_to_struct("TestJson", RustVisibility::Public)
            .set_visibility_to_struct("TestJsonDataEntities", RustVisibility::Public)
            .build("TestJson");

        assert_eq!(rust.gen_from_json(json), tobe);
    }
}
