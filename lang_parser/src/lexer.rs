use std::str::Chars;

use crate::token::Token;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Chars<'a>,
    position: usize,
}
impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let chars = input.chars();
        Self {
            input: chars,
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        None
    }
    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }
    fn skip_whitespace(&mut self) -> Option<char> {
         loop {
              let Some(c) = self.input.next() else {
                  return None;
              };
              if !c.is_whitespace() {
                  return Some(c);
              }
         }
    }
    fn peek_char(&mut self)->Option<char> {
        self.input.by_ref().peekable().peek().copied()
    }
    fn read_number(&mut self)-> String {
        let mut result = String::new(); 
        let  Some(mut ch) = self.read_char() else {
            return result; 
        }; 
        while Self::is_digit(ch) {
            result.push(ch);
            let Some(new_ch) = self.read_char() else {
                return result;
            };
            ch = new_ch;
        };
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
    fn white_skipが動作するか確認() {
        let input = "   hello world";
        let mut sut = Lexer::new(input);
        let c = sut.skip_whitespace();
        // let h = sut.input.next();
        assert_eq!(c.unwrap(),'h');
        let p = sut.peek_char();

        assert_eq!(p.unwrap(),'e');
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
        !-/*5;
        5 < 10 > 5;
        if(5<10) {
            return true;
        }else {
           return false; 
        }
        10 == 10;
        10 != 9;
        "#;

        let mut sut = Lexer::new(input);

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
            Token::new(TokenType::RBracket, ")"),
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
            Token::new(TokenType::Bang, "!"),
            Token::new(TokenType::Minus, "-"),
            Token::new(TokenType::Slash, "/"),
            Token::new(TokenType::Asterisk, "*"),
            Token::new(TokenType::NumberLiteral, "5"),
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
            Token::new(TokenType::RBracket, ")"),
            Token::new(TokenType::Else, "else"),
            Token::new(TokenType::LBracket, "{"),
            Token::new(TokenType::Return, "return"),
            Token::new(TokenType::False, "false"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::RBracket, ")"),
            Token::new(TokenType::NumberLiteral, "10"),
            Token::new(TokenType::Eq, "=="),
            Token::new(TokenType::NumberLiteral, "10"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::NumberLiteral, "10"),
            Token::new(TokenType::NotEq, "!="),
            Token::new(TokenType::NumberLiteral, "9"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Eof, ""),
        ];
        for t in tobe {
            let token = sut.next_token();
            assert_eq!(t, token.unwrap());
        }
    }
}
