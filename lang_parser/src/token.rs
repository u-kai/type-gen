use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub(crate) r#type: TokenType,
    pub(crate) literal: String,
}
impl Token {
    pub fn new(r#type: TokenType, literal: impl Into<String>) -> Self {
        Token {
            r#type,
            literal: literal.into(),
        }
    }
    pub fn eof() -> Self {
        Token::new(TokenType::Eof, "")
    }
    pub fn from_token_char(c: char) -> Self {
        match c {
            '/' => Token::new(TokenType::Slash, c),
            '*' => Token::new(TokenType::Asterisk, c),
            '<' => Token::new(TokenType::Lt, c),
            '>' => Token::new(TokenType::Gt, c),
            ',' => Token::new(TokenType::Comma, c),
            ';' => Token::new(TokenType::Semicolon, c),
            '(' => Token::new(TokenType::LParentheses, c),
            ')' => Token::new(TokenType::RParentheses, c),
            '{' => Token::new(TokenType::LBracket, c),
            '}' => Token::new(TokenType::RBracket, c),
            '=' => Token::new(TokenType::Assign, c),
            '!' => Token::new(TokenType::Bang, c),
            '+' => Token::new(TokenType::Plus, c),
            '-' => Token::new(TokenType::Minus, c),
            ':' => Token::new(TokenType::Colon, c),
            _ => Token::new(TokenType::Ident, c),
        }
    }
    pub fn from_ident(keywords: &KeywordsToTokenType, ident: &str) -> Self {
        let Some(token_type) = keywords.inner.get(ident) else {
            return Self::new(
                TokenType::Ident,
                ident)
        };
        Self::new(*token_type, ident)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Comment,
    Increment,
    Add,
    Sub,
    Colon,
    Decrement,
    Illegal,
    Assign,
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
#[derive(Debug)]
pub struct KeywordsToTokenType {
    inner: HashMap<&'static str, TokenType>,
    comment_ident: String,
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
        Self {
            inner,
            comment_ident: "//".to_string(),
        }
    }
    pub fn set_comment_ident(&mut self, comment_ident: impl Into<String>) {
        let comment_ident = comment_ident.into();
        self.comment_ident = comment_ident;
    }
    pub fn contain_comment_ident(&mut self, ch: char) -> bool {
        self.comment_ident.contains(ch)
    }
    pub fn get_comment_ident(&self) -> &str {
        &self.comment_ident
    }
    pub fn get(&self, word: &str) -> Option<TokenType> {
        self.inner.get(word).copied()
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
