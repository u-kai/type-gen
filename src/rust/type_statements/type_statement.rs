use crate::{
    lang_common::{filed_comment::BaseFiledComment, type_comment::BaseTypeComment},
    rust::off_side_rule::RustOffSideRule,
    traits::{
        off_side_rule::OffSideRule,
        type_statements::{
            type_attr::TypeAttribution, type_comment::TypeComment, type_statement::TypeStatement,
            type_visibility::TypeVisibility,
        },
    },
};

use super::{
    type_attr::{RustTypeAttribute, RustTypeAttributeStore},
    type_visiblity::RustTypeVisibilityProvider,
};

pub struct RustTypeStatement {
    comment: BaseTypeComment,
    attr: RustTypeAttributeStore,
    visi: RustTypeVisibilityProvider,
    off_side_rule: RustOffSideRule,
}

impl RustTypeStatement {
    const TYPE_STATEMENT: &'static str = "struct";
    pub fn new(
        comment: BaseTypeComment,
        attr: RustTypeAttributeStore,
        visi: RustTypeVisibilityProvider,
        off_side_rule: RustOffSideRule,
    ) -> Self {
        Self {
            comment,
            attr,
            visi,
            off_side_rule,
        }
    }
}

impl
    TypeStatement<
        BaseTypeComment,
        RustTypeAttributeStore,
        RustTypeVisibilityProvider,
        RustOffSideRule,
    > for RustTypeStatement
{
    fn create_statement(&self, type_key: &str) -> String {
        let visi = self.visi.get_visibility_str(type_key);
        let rule = self.off_side_rule.start();
        let mut result = format!("{}{} {} {}", visi, Self::TYPE_STATEMENT, type_key, rule);

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
            off_side_rule::RustOffSideRule,
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
    fn comment_and_attr_and_pub_visibilty() {
        let struct_name = "Test";
        let off_side_rule = RustOffSideRule::new();
        let mut comment = BaseTypeComment::new("//");
        comment.add_comment(struct_name, "this is test");
        comment.add_comment(struct_name, "hello");
        let mut attr = RustTypeAttributeStore::new();
        attr.set_attr(
            struct_name,
            RustTypeAttribute::from_derives(vec!["Debug", "Clone"]),
        );
        let mut visi = RustTypeVisibilityProvider::new();
        visi.add_visibility(struct_name, RustVisibility::Public);
        let tobe = r#"// this is test
// hello
#[derive(Debug,Clone)]
pub struct Test {
    "#;
        let rust = RustTypeStatement::new(comment, attr, visi, off_side_rule);
        assert_eq!(rust.create_statement(struct_name), tobe);
    }
}
