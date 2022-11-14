use std::cell::RefCell;

use npc::{convertor::NamingPrincipalConvertor, naming_principal::NamingPrincipal};

use crate::{
    langs::common::{
        filed_comment::BaseFiledComment,
        type_define_generators::{filed_key::FiledKey, filed_type::FiledType, type_key::TypeKey},
    },
    traits::filed_statements::{
        filed_attr::FiledAttribute, filed_comment::FiledComment, filed_statement::FiledStatement,
        filed_visibility::FiledVisibility, reserved_words::ReservedWords,
    },
};

use super::{
    filed_attr::RustFiledAttributeStore, filed_visibilty::RustFiledVisibilityProvider,
    reserved_words::RustReservedWords,
};

pub struct RustFiledStatement<'a> {
    comment: BaseFiledComment,
    attr: RefCell<RustFiledAttributeStore<'a>>,
    visi: RustFiledVisibilityProvider,
    reserved_words: RustReservedWords,
}
impl<'a> RustFiledStatement<'a> {
    pub fn new(
        comment: BaseFiledComment,
        attr: RefCell<RustFiledAttributeStore<'a>>,
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
impl<'a> FiledStatement for RustFiledStatement<'a> {
    fn create_statement(
        &self,
        type_key: &TypeKey,
        filed_key: &FiledKey,
        filed_type: &FiledType,
    ) -> String {
        let visi = self.visi.get_visibility_str(filed_key.value());
        let (new_key, rename_attr) = if !NamingPrincipal::is_snake(filed_key.value()) {
            let npc = NamingPrincipalConvertor::new(filed_key.value());
            let new_key = npc.to_snake();
            let rename_attr = format!("#[serde(rename = \"{}\")]\n    ", filed_key.value());
            (
                self.reserved_words.sub_or_default(&new_key).to_string(),
                Some(rename_attr),
            )
        } else {
            (
                self.reserved_words
                    .sub_or_default(filed_key.value())
                    .to_string(),
                None,
            )
        };
        let mut result = self.add_head_space(format!(
            "{}{}{}: {}{}\n",
            rename_attr.unwrap_or_default(),
            visi,
            new_key,
            filed_type.value(),
            Self::FILED_DERIMITA
        ));
        if let Some(attr) = self.attr.borrow().get_attr(type_key, filed_key) {
            result = self.add_head_space(format!("{}\n{}", attr, result));
        };
        if let Some(comments) = self.comment.get_comments(filed_key.value()) {
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
        langs::{
            common::{
                filed_comment::BaseFiledComment,
                type_define_generators::{
                    filed_key::FiledKey, filed_type::FiledType, type_key::TypeKey,
                },
            },
            rust::{
                filed_statements::{
                    filed_attr::{RustFiledAttribute, RustFiledAttributeStore},
                    filed_visibilty::RustFiledVisibilityProvider,
                    reserved_words::RustReservedWords,
                },
                rust_visibility::RustVisibility,
            },
        },
        traits::filed_statements::filed_statement::FiledStatement,
    };

    use super::RustFiledStatement;
    use std::cell::RefCell;

    #[test]
    fn pub_comment_and_attr_and_reserved_word() {
        let type_key = TypeKey::new("Test");
        let filed_key = FiledKey::new("type");
        let filed_type = FiledType::new("Option<String>");
        let mut comment = BaseFiledComment::new("//");
        comment.add_comment(filed_key.value(), "this is test");
        comment.add_comment(filed_key.value(), "hello");
        let mut visi = RustFiledVisibilityProvider::new();
        visi.add_visibility(filed_key.value(), RustVisibility::Public);
        let attr = RefCell::new(RustFiledAttributeStore::new());
        attr.borrow_mut().add_attr(
            &type_key,
            &filed_key,
            RustFiledAttribute::Original(String::from("cfg(not(test))")),
        );
        let reserved_words = RustReservedWords::new();
        let statement = RustFiledStatement::new(comment, attr, visi, reserved_words);
        let tobe = r#"    // this is test
    // hello
    #[cfg(not(test))]
    pub r#type: Option<String>,
"#;
        assert_eq!(
            statement.create_statement(&type_key, &filed_key, &filed_type,),
            tobe.to_string()
        );
    }
    #[test]
    fn pub_comment_and_attr_and() {
        let type_key = TypeKey::new("Test");
        let filed_key = FiledKey::new("test");
        let filed_type = FiledType::new("Option<String>");
        let mut comment = BaseFiledComment::new("//");
        comment.add_comment(filed_key.value(), "this is test");
        comment.add_comment(filed_key.value(), "hello");
        let mut visi = RustFiledVisibilityProvider::new();
        visi.add_visibility(filed_key.value(), RustVisibility::Public);
        let attr = RefCell::new(RustFiledAttributeStore::new());
        attr.borrow_mut().add_attr(
            &type_key,
            &filed_key,
            RustFiledAttribute::Original(String::from("cfg(not(test))")),
        );
        let reserved_words = RustReservedWords::new();
        let statement = RustFiledStatement::new(comment, attr, visi, reserved_words);
        let tobe = r#"    // this is test
    // hello
    #[cfg(not(test))]
    pub test: Option<String>,
"#;
        assert_eq!(
            statement.create_statement(&type_key, &filed_key, &filed_type),
            tobe.to_string()
        );
    }
}
