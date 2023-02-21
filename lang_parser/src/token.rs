use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Illegal,
    Eof,
    NumberLiteral,
    Ident,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Comma,
    Semicolon,
    LParentheses,
    RParentheses,
    LBracket,
    RBracket,
    Eq,
    NotEq,
    Fn,
    True,
    False,
    If,
    Else,
    Return,
    Let,
    Struct,
    Type,
    StringLiteral,
    Class,
}

pub struct KeywordsToTokenType {
    inner: HashMap<&'static str, TokenType>,
}

impl KeywordsToTokenType {
    pub fn new() -> Self {
        let mut inner = HashMap::new();
        inner.insert("fn", TokenType::Fn);
        inner.insert("let", TokenType::Let);
        inner.insert("return", TokenType::Return);
        inner.insert("if", TokenType::If);
        inner.insert("else", TokenType::Else);
        inner.insert("true", TokenType::True);
        inner.insert("false", TokenType::False);
        Self { inner }
    }
    pub fn add_type_keyword(&mut self) {
        self.inner.insert("type", TokenType::Type);
    }
    pub fn add_class_keyword(&mut self) {
        self.inner.insert("class", TokenType::Class);
    }
    pub fn add_struct_keyword(&mut self) {
        self.inner.insert("struct", TokenType::Struct);
    }
}
