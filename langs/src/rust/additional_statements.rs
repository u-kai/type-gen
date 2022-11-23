use lang_common::type_defines::additional_defines::comment_store::Comment;

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
