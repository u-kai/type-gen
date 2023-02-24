use crate::{
    ast::{Identifier, LetStatement, Program, Statement},
    lexer::Lexer,
    token::{Token, TokenType},
};

pub struct Parser<'a> {
    l: Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(l: Lexer<'a>) -> Self {
        let mut l = l;
        let cur_token = l.next_token();
        let peek_token = l.next_token();
        Self {
            l,
            cur_token,
            peek_token,
        }
    }
    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();
        while self.cur_token.r#type != TokenType::Eof {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            };
            self.set_next_token()
        }
        Program { statements }
    }
    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.r#type {
            TokenType::Let => self.parse_let_statement(),
            _ => None,
        }
    }
    fn parse_let_statement(&mut self) -> Option<Statement> {
        let stmt_token = self.cur_token.clone();
        if !self.expect_peek(TokenType::Ident) {
            return None;
        }
        let name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        while self.cur_token_is(TokenType::Semicolon) {
            self.set_next_token();
        }
        Some(Statement::LetStatement(LetStatement {
            token: stmt_token,
            name,
            expression: crate::ast::Expression::L,
        }))
    }
    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.set_next_token();
            true
        } else {
            false
        }
    }
    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.r#type == token_type
    }
    fn cur_token_is(&self, token_type: TokenType) -> bool {
        self.cur_token.r#type == token_type
    }
    fn set_next_token(&mut self) {
        std::mem::swap(&mut self.peek_token, &mut self.cur_token);
        self.peek_token = self.l.next_token();
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Node, Statement},
        lexer::Lexer,
    };

    use super::Parser;

    #[test]
    fn test_let_statements() {
        let input = r#"
            let x = 5;
            let y = 10;
            let foobar = 838383;
        "#;
        let lexer = Lexer::default(input);

        let mut sut = Parser::new(lexer);

        let program = sut.parse_program();

        let mut stmt = program.statements.into_iter();
        let stmt1 = stmt.next().unwrap();
        let stmt2 = stmt.next().unwrap();
        let stmt3 = stmt.next().unwrap();
        assert_let_statement(stmt1, "x");
        assert_let_statement(stmt2, "y");
        assert_let_statement(stmt3, "foobar");
        fn assert_let_statement(stmt: Statement, name: &str) {
            match stmt {
                Statement::LetStatement(s) => {
                    assert_eq!(s.string(), "let");
                    assert_eq!(s.name.value, name);
                }
                _ => panic!("not LetStatement, got={:#?}", stmt),
            }
        }
    }
}
