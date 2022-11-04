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

impl Into<&'static str> for RustVisibility {
    fn into(self) -> &'static str {
        match self {
            Self::Private => "",
            Self::Public => "pub",
            Self::PublicSuper => "pub(super)",
            Self::PubilcSelf => "pub(self)",
            Self::PublicCrate => "pub(crate)",
        }
    }
}

//impl From<RustVisibility> for &'static str {
//fn from(self) -> RustVisibility {
//match self {
//"" => RustVisibility::Private,
//"pub" => RustVisibility::Public,
//"pub(super)" => RustVisibility::PublicSuper,
//"pub(self)" => RustVisibility::PubilcSelf,
//"pub(crate)" => RustVisibility::PublicCrate,
//_ => panic!("not considering {}", self),
//}
//}
//}
