use std::cell::RefCell;

use crate::lang_common::{
    filed_comment::BaseFiledComment, optional_checker::BaseOptionalChecker,
    type_comment::BaseTypeComment, type_define_generator::TypeDefineGenerator,
};

use super::{
    filed_statements::{
        filed_attr::{RustFiledAttribute, RustFiledAttributeStore},
        filed_statement::RustFiledStatement,
        filed_visibilty::RustFiledVisibilityProvider,
        reserved_words::RustReservedWords,
    },
    json_lang_mapper::JsonRustMapper,
    off_side_rule::RustOffSideRule,
    rust_visibility::RustVisibility,
    type_statements::{
        type_attr::{RustTypeAttribute, RustTypeAttributeStore},
        type_statement::RustTypeStatement,
        type_visiblity::RustTypeVisibilityProvider,
    },
};

struct TypeStatements {
    comment: BaseTypeComment,
    visi: RustTypeVisibilityProvider,
    attr: RustTypeAttributeStore,
}
struct FiledStatements {
    comment: BaseFiledComment,
    visi: RustFiledVisibilityProvider,
    attr: RustFiledAttributeStore,
}
pub struct RustTypeGeneratorBuilder {
    type_statements: TypeStatements,
    filed_statements: FiledStatements,
    opsional_checker: BaseOptionalChecker,
}

impl RustTypeGeneratorBuilder {
    pub fn new() -> Self {
        let t = TypeStatements {
            comment: BaseTypeComment::new("//"),
            visi: RustTypeVisibilityProvider::new(),
            attr: RustTypeAttributeStore::new(),
        };
        let f = FiledStatements {
            comment: BaseFiledComment::new("//"),
            visi: RustFiledVisibilityProvider::new(),
            attr: RustFiledAttributeStore::new(),
        };
        Self {
            type_statements: t,
            filed_statements: f,
            opsional_checker: BaseOptionalChecker::default(),
        }
    }
    // build
    pub fn build(
        self,
        root_struct_name: &str,
    ) -> TypeDefineGenerator<JsonRustMapper, RustTypeStatement, RustFiledStatement, RustOffSideRule>
    {
        let mapper = JsonRustMapper::new();
        let type_statement = RustTypeStatement::new(
            self.type_statements.comment,
            self.type_statements.visi,
            self.type_statements.attr,
        );
        let filed_statement = RustFiledStatement::new(
            self.filed_statements.comment,
            RefCell::new(self.filed_statements.attr),
            self.filed_statements.visi,
            RustReservedWords::new(),
        );
        TypeDefineGenerator::new(
            root_struct_name,
            mapper,
            type_statement,
            filed_statement,
            RustOffSideRule::new(),
            self.opsional_checker,
        )
    }
    // visi
    pub fn set_visibility_to_all_struct(mut self, visi: RustVisibility) -> Self {
        self.type_statements.visi.set_all_visibility(visi);
        self
    }
    pub fn set_visibility_to_all_filed(mut self, visi: RustVisibility) -> Self {
        self.filed_statements.visi.set_all_visibility(visi);
        self
    }
    pub fn set_visibility_to_struct(mut self, struct_name: &str, visi: RustVisibility) -> Self {
        self.type_statements.visi.add_visibility(struct_name, visi);
        self
    }
    pub fn set_visibility_to_filed(mut self, filed_key: &str, visi: RustVisibility) -> Self {
        self.filed_statements.visi.add_visibility(filed_key, visi);
        self
    }
    // attr
    pub fn set_attr_to_all_struct(mut self, attrs: Vec<RustTypeAttribute>) -> Self {
        self.type_statements.attr.set_attr_all(attrs);
        self
    }
    pub fn set_attr_to_all_filed(mut self, attrs: Vec<RustFiledAttribute>) -> Self {
        self.filed_statements.attr.set_attr_all(attrs);
        self
    }
    pub fn add_attr_to_struct(mut self, struct_name: &str, attr: RustTypeAttribute) -> Self {
        self.type_statements.attr.add_attr(struct_name, attr);
        self
    }
    pub fn add_attr_to_filed(mut self, filed_key: &str, attr: RustFiledAttribute) -> Self {
        self.filed_statements.attr.add_attr(filed_key, attr);
        self
    }
    // comment
    pub fn add_comment_to_struct(mut self, struct_name: &str, comment: &str) -> Self {
        self.type_statements
            .comment
            .add_comment(struct_name, comment);
        self
    }
    pub fn add_comment_to_filed(mut self, filed_key: &str, comment: &str) -> Self {
        self.filed_statements
            .comment
            .add_comment(filed_key, comment);
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
    pub fn add_require(mut self, struct_name: &'static str, filed_key: &'static str) -> Self {
        self.opsional_checker.add_require(struct_name, filed_key);
        self
    }
    pub fn add_optional(mut self, struct_name: &'static str, filed_key: &'static str) -> Self {
        self.opsional_checker.add_optional(struct_name, filed_key);
        self
    }
}

#[cfg(test)]
mod test_rust_type_gen_builder {
    use crate::rust::{
        rust_visibility::RustVisibility, type_gen_builder::RustTypeGeneratorBuilder,
        type_statements::type_attr::RustTypeAttribute,
    };

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
}"#
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
