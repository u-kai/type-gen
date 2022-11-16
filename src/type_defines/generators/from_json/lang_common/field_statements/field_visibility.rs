pub trait FieldVisibility {
    fn get_visibility_str(&self, field_key: &str) -> &'static str;
}
