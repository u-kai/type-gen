pub trait FiledComment {
    fn add_comment(&self, filed_key: &str, filed_statement: &mut String) -> Option<String>;
}
