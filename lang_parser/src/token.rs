use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
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

pub struct KeywordsToToken {
    inner: HashMap<&'static str, Token>,
}

impl KeywordsToToken {
    pub fn new() -> Self {
        let mut inner = HashMap::new();
        inner.insert("fn", Token::Fn);
        inner.insert("let", Token::Let);
        inner.insert("return", Token::Return);
        inner.insert("if", Token::If);
        inner.insert("else", Token::Else);
        inner.insert("true", Token::True);
        inner.insert("false", Token::False);
        Self { inner }
    }
    pub fn add_type_keyword(&mut self) {
        self.inner.insert("type", Token::Type);
    }
    pub fn add_class_keyword(&mut self) {
        self.inner.insert("class", Token::Class);
    }
    pub fn add_struct_keyword(&mut self) {
        self.inner.insert("struct", Token::Struct);
    }
}
