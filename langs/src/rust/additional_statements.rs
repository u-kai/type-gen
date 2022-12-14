use lang_common::type_defines::additional_defines::{
    comment_store::Comment, visibility_store::Visibility,
};

use super::property_generator::RUST_PROPERTY_HEAD_SPACE;

#[derive(Debug, Clone)]
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
impl<I> From<I> for RustComment
where
    I: Into<String>,
{
    fn from(source: I) -> Self {
        let str: String = source.into();
        Self(str.split("\n").map(|s| s.to_string()).collect())
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
    PublicSelf,
    PublicCrate,
}
impl RustVisibility {
    fn from_str(str: &str) -> Result<Self, String> {
        match str {
            "pub" | "public" | "Pub" | "Public" | "export" => Ok(Self::Public),
            "" | "private" | "Private" => Ok(Self::Private),
            "pub(self)" | "pub (self)" | "pub self" => Ok(Self::PublicSelf),
            "pub(super)" | "pub (super)" | "pub super" => Ok(Self::PublicSuper),
            "pub(crate)" | "pub (crate)" | "pub crate" => Ok(Self::PublicCrate),
            _ => Err(format!("{} is not define rust visibility", str)),
        }
    }
}

impl Default for RustVisibility {
    fn default() -> Self {
        Self::Private
    }
}
impl<T> From<T> for RustVisibility
where
    T: Into<String>,
{
    fn from(str: T) -> Self {
        let str: String = str.into();
        RustVisibility::from_str(&str).unwrap()
    }
}
impl Visibility for RustVisibility {
    fn to_type_define(&self) -> &'static str {
        match self {
            Self::Private => "",
            Self::Public => "pub ",
            Self::PublicSuper => "pub(super) ",
            Self::PublicSelf => "pub(self) ",
            Self::PublicCrate => "pub(crate) ",
        }
    }
    fn to_property_define(&self) -> &'static str {
        match self {
            Self::Private => "",
            Self::Public => "pub ",
            Self::PublicSuper => "pub(super) ",
            Self::PublicSelf => "pub(self) ",
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
