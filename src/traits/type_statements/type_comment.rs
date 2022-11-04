pub trait TypeComment {
    fn get_comment(&self, type_key: &str) -> Option<String>;
}
