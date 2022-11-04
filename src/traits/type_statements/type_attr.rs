pub trait TypeAttribution {
    fn get_attr(&self, type_key: &str) -> Option<&'static str>;
}
