pub trait OptionalChecker {
    fn is_optional(&self, filed_key: &str) -> bool;
}
