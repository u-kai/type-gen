use lang_common::type_defines::additional_defines::{
    comment_store::Comment, visibility_store::Visibility,
};

pub struct RustComment<'a>(&'a str);

impl<'a> Comment for RustComment<'a> {
    fn to_define(&self) -> String {
        format!("// {}", self.0)
    }
}

pub struct RustDocsComment<'a>(&'a str);

impl<'a> Comment for RustDocsComment<'a> {
    fn to_define(&self) -> String {
        format!("/// {}", self.0)
    }
}

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
    fn to_define(&self) -> &'static str {
        match self {
            Self::Private => "",
            Self::Public => "pub ",
            Self::PublicSuper => "pub(super) ",
            Self::PubilcSelf => "pub(self) ",
            Self::PublicCrate => "pub(crate) ",
        }
    }
    fn default_visibility() -> &'static str {
        ""
    }
}
