use std::str::Chars;

use crate::token::{KeywordsToTokenType, Token, TokenType};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Chars<'a>,
    focus: char,
    keywords: KeywordsToTokenType,
}
impl<'a> Lexer<'a> {
    const EOF_CHAR: char = ' ';
    pub fn new(input: &'a str, keywords: KeywordsToTokenType) -> Self {
        let chars = input.chars();
        Self {
            input: chars,
            keywords,
            focus: Self::EOF_CHAR,
        }
    }
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.focus {
            '/' | '*' | '<' | '>' | ',' | ';' | '(' | ')' | '{' | '}' | ':' | '#' => {
                let c = self.focus;
                if self.keywords.contain_comment_ident(c) {
                    if self.keywords.get_comment_ident().len() == 1 {
                        let token = Token::new(TokenType::Comment, self.focus);
                        self.set_next_char();
                        return token;
                    }
                    let mut literal = String::new();
                    for _ in 0..self.keywords.get_comment_ident().len() {
                        literal.push(c);
                        if self.focus != c {
                            return Token::from_token_char(c);
                        }
                        self.set_next_char();
                    }
                    self.set_next_char();
                    return Token::new(TokenType::Comment, literal);
                }

                let token = Token::from_token_char(self.focus);
                self.set_next_char();
                token
            }
            '!' => {
                if let Some(()) = self.read_char() {
                    if self.focus == '=' {
                        self.set_next_char();
                        self.set_next_char();
                        return Token::new(TokenType::NotEq, "!=");
                    }
                };
                let token = Token::from_token_char('!');
                self.set_next_char();
                token
            }
            '=' => {
                if let Some(()) = self.read_char() {
                    if self.focus == '=' {
                        self.set_next_char();
                        return Token::new(TokenType::Eq, "==");
                    }
                };
                Token::from_token_char('=')
            }
            '+' => {
                if let Some(()) = self.read_char() {
                    if self.focus == '=' {
                        self.set_next_char();
                        return Token::new(TokenType::Add, "+=");
                    }
                    if self.focus == '+' {
                        self.set_next_char();
                        return Token::new(TokenType::Increment, "++");
                    }
                }
                Token::from_token_char('+')
            }
            '-' => {
                if let Some(()) = self.read_char() {
                    if self.focus == '=' {
                        self.set_next_char();
                        return Token::new(TokenType::Sub, "-=");
                    }
                    if self.focus == '-' {
                        self.set_next_char();
                        return Token::new(TokenType::Decrement, "--");
                    }
                }
                Token::from_token_char('-')
            }
            _ => {
                if Self::is_letter(self.focus) {
                    let literal = self.read_identify();
                    if let Some(token_type) = self.keywords.get(&literal) {
                        return Token::new(token_type, literal);
                    }
                    return Token::new(TokenType::Ident, literal);
                }
                if Self::is_digit(self.focus) {
                    let literal = self.read_number();
                    return Token::new(TokenType::NumberLiteral, literal);
                }
                Token::eof()
            }
        }
    }
    fn set_next_char(&mut self) {
        if let Some(()) = self.read_char() {
            return;
        }
        self.focus = Self::EOF_CHAR;
    }
    fn read_char(&mut self) -> Option<()> {
        if let Some(c) = self.input.next() {
            self.focus = c;
            return Some(());
        }
        None
    }
    fn skip_whitespace(&mut self) {
        loop {
            if !self.focus.is_whitespace() {
                return;
            }
            if self.read_char().is_none() {
                return;
            };
        }
    }
    fn read_identify(&mut self) -> String {
        let mut result = String::new();
        while Self::is_letter(self.focus) {
            result.push(self.focus);
            self.set_next_char();
        }
        result
    }
    fn read_number(&mut self) -> String {
        let mut result = String::new();
        while Self::is_digit(self.focus) {
            result.push(self.focus);
            self.set_next_char();
        }
        result
    }
    fn is_digit(ch: char) -> bool {
        ch.is_numeric()
    }
    fn is_letter(ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Token, TokenType};

    #[test]
    fn 二文字がコメント文の識別子の場合コメント文の識別子としてtokenを認識できる() {
        let mut keywords = KeywordsToTokenType::new();
        keywords.add_struct_keyword();
        keywords.set_comment_ident("//");

        let input = r"data = 5
        // これはテストです
        // last";

        let mut sut = Lexer::new(input, keywords);

        assert_eq!(sut.next_token(), Token::new(TokenType::Ident, "data"));
        assert_eq!(sut.next_token(), Token::new(TokenType::Assign, "="));
        assert_eq!(sut.next_token(), Token::new(TokenType::NumberLiteral, "5"));
        assert_eq!(sut.next_token(), Token::new(TokenType::Comment, "//"));
        assert_eq!(
            sut.next_token(),
            Token::new(TokenType::Ident, "これはテストです")
        );
        assert_eq!(sut.next_token(), Token::new(TokenType::Comment, "//"));
        assert_eq!(sut.next_token(), Token::new(TokenType::Ident, "last"));
        assert_eq!(sut.next_token(), Token::new(TokenType::Eof, ""));
    }
    #[test]
    fn 一文字がコメント文の識別子の場合コメント文の識別子としてtokenを認識できる() {
        let mut keywords = KeywordsToTokenType::new();
        keywords.add_struct_keyword();
        keywords.set_comment_ident("#");

        let input = r"data = 5
        # これはテストです
        ";

        let mut sut = Lexer::new(input, keywords);

        assert_eq!(sut.next_token(), Token::new(TokenType::Ident, "data"));
        assert_eq!(sut.next_token(), Token::new(TokenType::Assign, "="));
        assert_eq!(sut.next_token(), Token::new(TokenType::NumberLiteral, "5"));
        assert_eq!(sut.next_token(), Token::new(TokenType::Comment, "#"));
        assert_eq!(
            sut.next_token(),
            Token::new(TokenType::Ident, "これはテストです")
        );
        assert_eq!(sut.next_token(), Token::new(TokenType::Eof, ""));
    }
    #[test]
    fn keywordsにstructを追加することでtokenの配列を生成することができる() {
        let mut keywords = KeywordsToTokenType::new();
        keywords.add_struct_keyword();

        let input = r#"struct Test {
            id:usize
        }"#;

        let mut sut = Lexer::new(input, keywords);

        assert_eq!(sut.next_token(), Token::new(TokenType::Struct, "struct"));
        assert_eq!(sut.next_token(), Token::new(TokenType::Ident, "Test"));
        assert_eq!(sut.next_token(), Token::new(TokenType::LBracket, "{"));
        assert_eq!(sut.next_token(), Token::new(TokenType::Ident, "id"));
        assert_eq!(sut.next_token(), Token::new(TokenType::Colon, ":"));
        assert_eq!(sut.next_token(), Token::new(TokenType::Ident, "usize"));
        assert_eq!(sut.next_token(), Token::new(TokenType::RBracket, "}"));
        assert_eq!(sut.next_token(), Token::new(TokenType::Eof, ""));
    }
    #[test]
    fn 基本的な構文に対してtokenの配列を生成することができる() {
        let input = r#"
        let five = 5;
        let ten = 10;
        let add = fn (x,y) {
            x+y;
        };
        let result = add(five,ten);
        5 < 10 > 5;
        if(5<10) {
            return true;
        }else {
           return false; 
        }
        10 == 10;
        10 != 9;
        *5;
        /5;
        "#;

        let mut sut = Lexer::new(input, KeywordsToTokenType::new());

        let tobe = vec![
            Token::new(TokenType::Let, "let"),
            Token::new(TokenType::Ident, "five"),
            Token::new(TokenType::Assign, "="),
            Token::new(TokenType::NumberLiteral, "5"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Let, "let"),
            Token::new(TokenType::Ident, "ten"),
            Token::new(TokenType::Assign, "="),
            Token::new(TokenType::NumberLiteral, "10"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Let, "let"),
            Token::new(TokenType::Ident, "add"),
            Token::new(TokenType::Assign, "="),
            Token::new(TokenType::Fn, "fn"),
            Token::new(TokenType::LParentheses, "("),
            Token::new(TokenType::Ident, "x"),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::Ident, "y"),
            Token::new(TokenType::RParentheses, ")"),
            Token::new(TokenType::LBracket, "{"),
            Token::new(TokenType::Ident, "x"),
            Token::new(TokenType::Plus, "+"),
            Token::new(TokenType::Ident, "y"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::RBracket, "}"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Let, "let"),
            Token::new(TokenType::Ident, "result"),
            Token::new(TokenType::Assign, "="),
            Token::new(TokenType::Ident, "add"),
            Token::new(TokenType::LParentheses, "("),
            Token::new(TokenType::Ident, "five"),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::Ident, "ten"),
            Token::new(TokenType::RParentheses, ")"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::NumberLiteral, "5"),
            Token::new(TokenType::Lt, "<"),
            Token::new(TokenType::NumberLiteral, "10"),
            Token::new(TokenType::Gt, ">"),
            Token::new(TokenType::NumberLiteral, "5"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::If, "if"),
            Token::new(TokenType::LParentheses, "("),
            Token::new(TokenType::NumberLiteral, "5"),
            Token::new(TokenType::Lt, "<"),
            Token::new(TokenType::NumberLiteral, "10"),
            Token::new(TokenType::RParentheses, ")"),
            Token::new(TokenType::LBracket, "{"),
            Token::new(TokenType::Return, "return"),
            Token::new(TokenType::True, "true"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::RBracket, "}"),
            Token::new(TokenType::Else, "else"),
            Token::new(TokenType::LBracket, "{"),
            Token::new(TokenType::Return, "return"),
            Token::new(TokenType::False, "false"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::RBracket, "}"),
            Token::new(TokenType::NumberLiteral, "10"),
            Token::new(TokenType::Eq, "=="),
            Token::new(TokenType::NumberLiteral, "10"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::NumberLiteral, "10"),
            Token::new(TokenType::NotEq, "!="),
            Token::new(TokenType::NumberLiteral, "9"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Asterisk, "*"),
            Token::new(TokenType::NumberLiteral, "5"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Slash, "/"),
            Token::new(TokenType::NumberLiteral, "5"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Eof, ""),
        ];
        for (i, t) in tobe.into_iter().enumerate() {
            let token = sut.next_token();
            println!("i:{}", i);
            assert_eq!(token, t);
        }
    }
}
