use crate::{
    lang_common::type_comment::BaseTypeComment, rust::off_side_rule::RustOffSideRule,
    traits::type_statements::type_statement::TypeStatement,
};

use super::{type_attr::RustTypeAttributeStore, type_visiblity::RustTypeVisibilityProvider};

pub struct RustTypeStatement {}

impl RustTypeStatement {
    pub fn new() -> Self {
        Self {}
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
    const TYPE_STATEMENT: &'static str = "struct";
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
    fn only_private_struct() {
        let struct_name = "Test";
        let rust = RustTypeStatement::new();
        let tobe = r#"struct Test {
"#;
        let off_side_rule = RustOffSideRule::new();
        let comment = BaseTypeComment::new("//");
        let attr = RustTypeAttributeStore::new();
        let visi = RustTypeVisibilityProvider::new();
        assert_eq!(
            rust.create_statement(struct_name, &comment, &attr, &visi, &off_side_rule),
            tobe.to_string()
        );
    }
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
        let rust = RustTypeStatement::new();
        assert_eq!(
            rust.create_statement(struct_name, &comment, &attr, &visi, &off_side_rule),
            tobe
        );
    }
}
