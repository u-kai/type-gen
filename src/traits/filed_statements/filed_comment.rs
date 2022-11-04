pub trait FiledComment {
    fn get_comment(&self, filed_key: &str) -> Option<&str>;
}
