pub trait FieldVisibility {
    fn get_visibility_str(&self, type_key: &str, field_key: &str) -> &'static str;
}
