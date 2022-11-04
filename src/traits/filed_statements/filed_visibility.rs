pub trait FiledVisibility {
    fn get_visibility_str(&self, filed_key: &str) -> &'static str;
}
