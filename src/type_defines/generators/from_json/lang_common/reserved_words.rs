pub trait ReservedWords {
    fn sub_or_default<'a>(&self, word: &'a str) -> &'a str;
}
