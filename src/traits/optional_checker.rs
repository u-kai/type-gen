pub trait OptionalChecker {
    fn is_optional(&self, type_key: &str, filed_key: &str) -> bool;
}
