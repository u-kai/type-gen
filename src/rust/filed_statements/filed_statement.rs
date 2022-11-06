use npc::{convertor::NamingPrincipalConvertor, naming_principal::NamingPrincipal};

use crate::{
    lang_common::filed_comment::BaseFiledComment,
    rust::reserved_words::RustReservedWords,
    traits::filed_statements::{
        filed_attr::FiledAttribute, filed_comment::FiledComment, filed_statement::FiledStatement,
        filed_visibility::FiledVisibility, reserved_words::ReservedWords,
    },
};

use super::{
    filed_attr::{RustFiledAttribute, RustFiledAttributeStore},
    filed_visibilty::RustFiledVisibilityProvider,
};

pub struct RustFiledStatement {}
impl RustFiledStatement {
    pub fn new() -> Self {
        Self {}
    }
}
impl
    FiledStatement<
        BaseFiledComment,
        RustFiledAttributeStore,
        RustFiledVisibilityProvider,
        RustReservedWords,
    > for RustFiledStatement
{
    fn create_statement(
        &self,
        filed_key: &str,
        filed_type: &str,
        comment: &BaseFiledComment,
        attr: &mut RustFiledAttributeStore,
        visi: &RustFiledVisibilityProvider,
        reserved_words: &RustReservedWords,
    ) -> String {
        let visi = visi.get_visibility_str(filed_key);
        let new_key = if !NamingPrincipal::is_snake(filed_key) {
            let npc = NamingPrincipalConvertor::new(filed_key);
            let new_key = npc.to_snake();
            attr.add_attr(
                &filed_key,
                RustFiledAttribute::Original(format!(r#"serde(rename = "{}")"#, filed_key)),
            );
            reserved_words.sub_or_default(&new_key).to_string()
        } else {
            reserved_words.sub_or_default(filed_key).to_string()
        };
        let mut result = self.add_head_space(format!(
            "{}{}: {}{}",
            visi,
            new_key,
            filed_type,
            Self::FILED_DERIMITA
        ));
        if let Some(attr) = attr.get_attr(filed_key) {
            result = self.add_head_space(format!("{}\n{}", attr, result));
        };
        if let Some(comments) = comment.get_comments(filed_key) {
            for comment in comments.iter().rev() {
                result = format!("{}{}\n{}", Self::HEAD_SPACE, comment, result);
            }
        };
        result
    }
}
#[cfg(test)]
mod test_rust_filed_statement {

    use crate::{
        lang_common::filed_comment::BaseFiledComment,
        rust::{
            filed_statements::{
                filed_attr::{RustFiledAttribute, RustFiledAttributeStore},
                filed_visibilty::RustFiledVisibilityProvider,
            },
            reserved_words::RustReservedWords,
            rust_visibility::RustVisibility,
        },
        traits::filed_statements::filed_statement::FiledStatement,
    };

    use super::RustFiledStatement;

    #[test]
    fn pub_comment_and_attr_and_reserved_word() {
        let filed_key = "type";
        let filed_type = "Option<String>";
        let mut comment = BaseFiledComment::new("//");
        comment.add_comment(filed_key, "this is test");
        comment.add_comment(filed_key, "hello");
        let mut visi = RustFiledVisibilityProvider::new();
        visi.add_visibility(filed_key, RustVisibility::Public);
        let mut attr = RustFiledAttributeStore::new();
        attr.add_attr(
            filed_key,
            RustFiledAttribute::Original(String::from("cfg(not(test))")),
        );
        let statement = RustFiledStatement::new();
        let reserved_words = RustReservedWords::new();
        let tobe = r#"    // this is test
    // hello
    #[cfg(not(test))]
    pub r#type: Option<String>,"#;
        assert_eq!(
            statement.create_statement(
                filed_key,
                filed_type,
                &comment,
                &mut attr,
                &visi,
                &reserved_words
            ),
            tobe.to_string()
        );
    }
    #[test]
    fn pub_comment_and_attr_and() {
        let filed_key = "test";
        let filed_type = "Option<String>";
        let mut comment = BaseFiledComment::new("//");
        comment.add_comment(filed_key, "this is test");
        comment.add_comment(filed_key, "hello");
        let mut visi = RustFiledVisibilityProvider::new();
        visi.add_visibility(filed_key, RustVisibility::Public);
        let mut attr = RustFiledAttributeStore::new();
        attr.add_attr(
            filed_key,
            RustFiledAttribute::Original(String::from("cfg(not(test))")),
        );
        let statement = RustFiledStatement::new();
        let reserved_words = RustReservedWords::new();
        let tobe = r#"    // this is test
    // hello
    #[cfg(not(test))]
    pub test: Option<String>,"#;
        assert_eq!(
            statement.create_statement(
                filed_key,
                filed_type,
                &comment,
                &mut attr,
                &visi,
                &reserved_words
            ),
            tobe.to_string()
        );
    }
}
