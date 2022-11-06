use std::cell::RefCell;

use npc::{convertor::NamingPrincipalConvertor, naming_principal::NamingPrincipal};

use crate::{
    lang_common::filed_comment::BaseFiledComment,
    traits::filed_statements::{
        filed_attr::FiledAttribute, filed_comment::FiledComment, filed_statement::FiledStatement,
        filed_visibility::FiledVisibility, reserved_words::ReservedWords,
    },
};

use super::{
    filed_attr::{RustFiledAttribute, RustFiledAttributeStore},
    filed_visibilty::RustFiledVisibilityProvider,
    reserved_words::RustReservedWords,
};

pub struct RustFiledStatement {
    comment: BaseFiledComment,
    attr: RefCell<RustFiledAttributeStore>,
    visi: RustFiledVisibilityProvider,
    reserved_words: RustReservedWords,
}
impl RustFiledStatement {
    pub fn new(
        comment: BaseFiledComment,
        attr: RefCell<RustFiledAttributeStore>,
        visi: RustFiledVisibilityProvider,
        reserved_words: RustReservedWords,
    ) -> Self {
        Self {
            comment,
            attr,
            visi,
            reserved_words,
        }
    }
}
impl FiledStatement for RustFiledStatement {
    fn create_statement(&self, filed_key: &str, filed_type: &str) -> String {
        let visi = self.visi.get_visibility_str(filed_key);
        let new_key = if !NamingPrincipal::is_snake(filed_key) {
            let npc = NamingPrincipalConvertor::new(filed_key);
            let new_key = npc.to_snake();
            self.attr.borrow_mut().add_attr(
                &filed_key,
                RustFiledAttribute::Original(format!(r#"serde(rename = "{}")"#, filed_key)),
            );
            self.reserved_words.sub_or_default(&new_key).to_string()
        } else {
            self.reserved_words.sub_or_default(filed_key).to_string()
        };
        let mut result = self.add_head_space(format!(
            "{}{}: {}{}",
            visi,
            new_key,
            filed_type,
            Self::FILED_DERIMITA
        ));
        if let Some(attr) = self.attr.borrow().get_attr(filed_key) {
            result = self.add_head_space(format!("{}\n{}", attr, result));
        };
        if let Some(comments) = self.comment.get_comments(filed_key) {
            for comment in comments.iter().rev() {
                result = format!("{}{}\n{}", Self::HEAD_SPACE, comment, result);
            }
        };
        result
    }
}
#[cfg(test)]
mod test_rust_filed_statement {

    use std::cell::RefCell;

    use crate::{
        lang_common::filed_comment::BaseFiledComment,
        rust::{
            filed_statements::{
                filed_attr::{RustFiledAttribute, RustFiledAttributeStore},
                filed_visibilty::RustFiledVisibilityProvider,
                reserved_words::RustReservedWords,
            },
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
        let attr = RefCell::new(RustFiledAttributeStore::new());
        attr.borrow_mut().add_attr(
            filed_key,
            RustFiledAttribute::Original(String::from("cfg(not(test))")),
        );
        let reserved_words = RustReservedWords::new();
        let statement = RustFiledStatement::new(comment, attr, visi, reserved_words);
        let tobe = r#"    // this is test
    // hello
    #[cfg(not(test))]
    pub r#type: Option<String>,"#;
        assert_eq!(
            statement.create_statement(filed_key, filed_type,),
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
        let attr = RefCell::new(RustFiledAttributeStore::new());
        attr.borrow_mut().add_attr(
            filed_key,
            RustFiledAttribute::Original(String::from("cfg(not(test))")),
        );
        let reserved_words = RustReservedWords::new();
        let statement = RustFiledStatement::new(comment, attr, visi, reserved_words);
        let tobe = r#"    // this is test
    // hello
    #[cfg(not(test))]
    pub test: Option<String>,"#;
        assert_eq!(
            statement.create_statement(filed_key, filed_type,),
            tobe.to_string()
        );
    }
}
