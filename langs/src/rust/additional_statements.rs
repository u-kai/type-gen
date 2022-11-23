use lang_common::type_defines::additional_defines::{
    comment_store::Comment, visibility_store::Visibility,
};

use super::property_generator::RUST_PROPERTY_HEAD_SPACE;

pub struct RustComment(Vec<String>);
impl RustComment {
    const COMMENT_PREFIX: &'static str = "// ";
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn add_comment_line(&mut self, comment: impl Into<String>) {
        self.0.push(comment.into());
    }
}

impl<'a> Comment for RustComment {
    fn to_property_define(&self) -> String {
        self.0.iter().fold(String::new(), |acc, cur| {
            format!(
                "{acc}{head}{prefix}{comment}{next_line}",
                acc = acc,
                head = RUST_PROPERTY_HEAD_SPACE,
                prefix = Self::COMMENT_PREFIX,
                comment = cur,
                next_line = "\n"
            )
        })
    }
    fn to_type_define(&self) -> String {
        self.0.iter().fold(String::new(), |acc, cur| {
            format!(
                "{acc}{prefix}{comment}{next_line}",
                acc = acc,
                prefix = Self::COMMENT_PREFIX,
                comment = cur,
                next_line = "\n"
            )
        })
    }
}

//pub struct RustDocsComment<'a>(&'a str);

//impl<'a> Comment for RustDocsComment<'a> {
//fn to_define(&self) -> String {
//format!("/// {}", self.0)
//}
//}

#[derive(Debug, Clone, Copy)]
pub enum RustVisibility {
    Private,
    Public,
    PublicSuper,
    PubilcSelf,
    PublicCrate,
}
impl Default for RustVisibility {
    fn default() -> Self {
        Self::Private
    }
}
impl Visibility for RustVisibility {
    fn to_type_define(&self) -> &'static str {
        match self {
            Self::Private => "",
            Self::Public => "pub ",
            Self::PublicSuper => "pub(super) ",
            Self::PubilcSelf => "pub(self) ",
            Self::PublicCrate => "pub(crate) ",
        }
    }
    fn to_property_define(&self) -> &'static str {
        match self {
            Self::Private => "",
            Self::Public => "pub ",
            Self::PublicSuper => "pub(super) ",
            Self::PubilcSelf => "pub(self) ",
            Self::PublicCrate => "pub(crate) ",
        }
    }
    fn default_type_visibility() -> &'static str {
        Self::to_type_define(&Self::Private)
    }
    fn default_property_visibility() -> &'static str {
        Self::to_property_define(&Self::Private)
    }
}
