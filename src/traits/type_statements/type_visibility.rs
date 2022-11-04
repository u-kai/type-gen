pub trait TypeVisibility {
    fn get_visibility_str(&self, type_key: &str) -> &'static str;
}
