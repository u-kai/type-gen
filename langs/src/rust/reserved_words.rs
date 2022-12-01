#[derive(Debug, Clone)]

pub struct RustReservedWords {
    reserved: [&'static str; 46],
    strict: [&'static str; 7],
}
impl RustReservedWords {
    pub fn new() -> Self {
        let reserved = [
            "as", "async", "await", "break", "continue", "else", "enum", "false", "true", "fn",
            "const", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut",
            "pub", "ref", "return", "static", "struct", "super", "trait", "true", "type", "unsafe",
            "where", "while", "abstract", "become", "box", "do", "final", "macro", "override",
            "priv", "try", "typeof", "unsized", "virtual", "yield",
        ];
        let strict = ["extern", "Self", "self", "use", "crate", "_", "super"];
        Self { reserved, strict }
    }
    pub fn is_reserved_keywords(&self, word: &str) -> bool {
        self.reserved.contains(&word)
    }
    pub fn is_strict_keywords(&self, word: &str) -> bool {
        self.strict.contains(&word)
    }
}

#[cfg(test)]
mod test_rust_reserved_words {
    use super::RustReservedWords;

    #[test]
    fn test_get_or_origin() {
        let reserved_words = RustReservedWords::new();
        assert_eq!(reserved_words.is_reserved_keywords("type"), true);
        assert_eq!(reserved_words.is_strict_keywords("super"), true);
        assert_eq!(reserved_words.is_reserved_keywords("data"), false);
        assert_eq!(reserved_words.is_strict_keywords("data"), false);
    }
}
