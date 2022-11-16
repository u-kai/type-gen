use std::cell::RefCell;

use npc::naming_principal::NamingPrincipal;

use crate::type_defines::{
    generators::from_json::lang_common::{
        field_statements::{
            field_attr::FieldAttribute,
            field_comment::{BaseFieldComment, FieldComment},
            field_statement::FieldStatement,
            field_visibility::FieldVisibility,
        },
        reserved_words::ReservedWords,
    },
    statement_parts::{field_key::Fieldkey, field_type::FieldType, type_key::TypeKey},
};

use super::{
    field_attr::RustFieldAttributeStore, field_visibility::RustFieldVisibilityProvider,
    reserved_words::RustReservedWords,
};

pub struct RustfieldStatement<'a> {
    comment: BaseFieldComment,
    attr: RefCell<RustFieldAttributeStore<'a>>,
    visi: RustFieldVisibilityProvider,
    reserved_words: RustReservedWords,
}
impl<'a> RustfieldStatement<'a> {
    pub fn new(
        comment: BaseFieldComment,
        attr: RefCell<RustFieldAttributeStore<'a>>,
        visi: RustFieldVisibilityProvider,
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
impl<'a> FieldStatement for RustfieldStatement<'a> {
    fn create_statement(
        &self,
        type_key: &TypeKey,
        field_key: &Fieldkey,
        field_type: &FieldType,
    ) -> String {
        let visi = self.visi.get_visibility_str(field_key.original());
        let (new_key, rename_attr) = if !NamingPrincipal::is_snake(field_key.original()) {
            let new_key =
                field_key.rename(crate::type_defines::generators::from_json::lang_common::naming_principal::NamingPrincipal::Snake);
            let rename_attr = format!("#[serde(rename = \"{}\")]\n    ", field_key.original());
            (
                self.reserved_words.sub_or_default(&new_key).to_string(),
                Some(rename_attr),
            )
        } else {
            (
                self.reserved_words
                    .sub_or_default(field_key.original())
                    .to_string(),
                None,
            )
        };
        let mut result = self.add_head_space(format!(
            "{}{}{}: {}{}\n",
            rename_attr.unwrap_or_default(),
            visi,
            new_key,
            field_type.value(),
            Self::FIELD_DERIMITA
        ));
        if let Some(attr) = self.attr.borrow().get_attr(type_key, field_key) {
            result = self.add_head_space(format!("{}\n{}", attr, result));
        };
        if let Some(comments) = self.comment.get_comments(field_key.original()) {
            for comment in comments.iter().rev() {
                result = format!("{}{}\n{}", Self::HEAD_SPACE, comment, result);
            }
        };
        result
    }
}
#[cfg(test)]
mod test_rust_field_statement {

    use crate::type_defines::{
        generators::from_json::{
            lang_common::field_statements::{
                field_comment::BaseFieldComment, field_statement::FieldStatement,
            },
            rust::{
                field_statements::{
                    field_attr::{RustFieldAttribute, RustFieldAttributeStore},
                    field_visibility::RustFieldVisibilityProvider,
                    reserved_words::RustReservedWords,
                },
                rust_visibility::RustVisibility,
            },
        },
        statement_parts::{field_key::Fieldkey, field_type::FieldType, type_key::TypeKey},
    };

    use super::RustfieldStatement;
    use std::cell::RefCell;

    #[test]
    fn pub_comment_and_attr_and_reserved_word_and_use_cannot_used() {
        let type_key = TypeKey::new("Test");
        let field_key = Fieldkey::new("cannot:Use");
        let field_type = FieldType::new("Option<String>");
        let mut comment = BaseFieldComment::new("//");
        comment.add_comment(field_key.original(), "this is test");
        comment.add_comment(field_key.original(), "hello");
        let mut visi = RustFieldVisibilityProvider::new();
        visi.add_visibility(field_key.original(), RustVisibility::Public);
        let attr = RefCell::new(RustFieldAttributeStore::new());
        attr.borrow_mut().add_attr(
            &type_key,
            &field_key,
            RustFieldAttribute::Original(String::from("cfg(not(test))")),
        );
        let reserved_words = RustReservedWords::new();
        let statement = RustfieldStatement::new(comment, attr, visi, reserved_words);
        let tobe = r#"    // this is test
    // hello
    #[cfg(not(test))]
    #[serde(rename = "cannot:Use")]
    pub cannot_use: Option<String>,
"#;
        assert_eq!(
            statement.create_statement(&type_key, &field_key, &field_type,),
            tobe.to_string()
        );
    }
    #[test]
    fn pub_comment_and_attr_and_reserved_word() {
        let type_key = TypeKey::new("Test");
        let field_key = Fieldkey::new("type");
        let field_type = FieldType::new("Option<String>");
        let mut comment = BaseFieldComment::new("//");
        comment.add_comment(field_key.original(), "this is test");
        comment.add_comment(field_key.original(), "hello");
        let mut visi = RustFieldVisibilityProvider::new();
        visi.add_visibility(field_key.original(), RustVisibility::Public);
        let attr = RefCell::new(RustFieldAttributeStore::new());
        attr.borrow_mut().add_attr(
            &type_key,
            &field_key,
            RustFieldAttribute::Original(String::from("cfg(not(test))")),
        );
        let reserved_words = RustReservedWords::new();
        let statement = RustfieldStatement::new(comment, attr, visi, reserved_words);
        let tobe = r#"    // this is test
    // hello
    #[cfg(not(test))]
    pub r#type: Option<String>,
"#;
        assert_eq!(
            statement.create_statement(&type_key, &field_key, &field_type,),
            tobe.to_string()
        );
    }
    #[test]
    fn pub_comment_and_attr_and() {
        let type_key = TypeKey::new("Test");
        let field_key = Fieldkey::new("test");
        let field_type = FieldType::new("Option<String>");
        let mut comment = BaseFieldComment::new("//");
        comment.add_comment(field_key.original(), "this is test");
        comment.add_comment(field_key.original(), "hello");
        let mut visi = RustFieldVisibilityProvider::new();
        visi.add_visibility(field_key.original(), RustVisibility::Public);
        let attr = RefCell::new(RustFieldAttributeStore::new());
        attr.borrow_mut().add_attr(
            &type_key,
            &field_key,
            RustFieldAttribute::Original(String::from("cfg(not(test))")),
        );
        let reserved_words = RustReservedWords::new();
        let statement = RustfieldStatement::new(comment, attr, visi, reserved_words);
        let tobe = r#"    // this is test
    // hello
    #[cfg(not(test))]
    pub test: Option<String>,
"#;
        assert_eq!(
            statement.create_statement(&type_key, &field_key, &field_type),
            tobe.to_string()
        );
    }
}
