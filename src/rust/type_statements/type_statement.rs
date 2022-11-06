use crate::{
    lang_common::type_comment::BaseTypeComment,
    traits::type_statements::{
        type_attr::TypeAttribution, type_comment::TypeComment, type_statement::TypeStatement,
        type_visibility::TypeVisibility,
    },
};

use super::{type_attr::RustTypeAttributeStore, type_visiblity::RustTypeVisibilityProvider};

pub struct RustTypeStatement {
    comment: BaseTypeComment,
    visi: RustTypeVisibilityProvider,
    attr: RustTypeAttributeStore,
}

impl RustTypeStatement {
    pub fn new(
        comment: BaseTypeComment,
        visi: RustTypeVisibilityProvider,
        attr: RustTypeAttributeStore,
    ) -> Self {
        Self {
            comment,
            visi,
            attr,
        }
    }
}

impl TypeStatement for RustTypeStatement {
    const TYPE_STATEMENT: &'static str = "struct";
    fn create_statement(&self, type_key: &str) -> String {
        let visi = self.visi.get_visibility_str(type_key);
        let mut result = format!("{}{} {}", visi, Self::TYPE_STATEMENT, type_key);

        if let Some(attr) = self.attr.get_attr(type_key) {
            result = format!("{}{}", attr, result);
        };
        if let Some(comment) = self.comment.get_comment(type_key) {
            result = format!("{}{}", comment, result);
        };
        result
    }
}

#[cfg(test)]
mod test_rust_type_statement {
    use crate::{
        lang_common::type_comment::BaseTypeComment,
        rust::{
            rust_visibility::RustVisibility,
            type_statements::{
                type_attr::{RustTypeAttribute, RustTypeAttributeStore},
                type_visiblity::RustTypeVisibilityProvider,
            },
        },
        traits::type_statements::type_statement::TypeStatement,
    };

    use super::RustTypeStatement;

    #[test]
    fn only_private_struct() {
        let struct_name = "Test";
        let comment = BaseTypeComment::new("//");
        let attr = RustTypeAttributeStore::new();
        let visi = RustTypeVisibilityProvider::new();
        let rust = RustTypeStatement::new(comment, visi, attr);
        let tobe = r#"struct Test"#;
        assert_eq!(rust.create_statement(struct_name), tobe.to_string());
    }
    #[test]
    fn comment_and_attr_and_pub_visibilty() {
        let struct_name = "Test";
        let mut comment = BaseTypeComment::new("//");
        comment.add_comment(struct_name, "this is test");
        comment.add_comment(struct_name, "hello");
        let mut attr = RustTypeAttributeStore::new();
        attr.add_attr(
            struct_name,
            RustTypeAttribute::from_derives(vec!["Debug", "Clone"]),
        );
        let mut visi = RustTypeVisibilityProvider::new();
        visi.add_visibility(struct_name, RustVisibility::Public);
        let tobe = r#"// this is test
// hello
#[derive(Debug,Clone)]
pub struct Test"#;
        let rust = RustTypeStatement::new(comment, visi, attr);
        assert_eq!(rust.create_statement(struct_name), tobe);
    }
}
