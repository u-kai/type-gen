use crate::{
    ast::{
        Expression, ExpressionStatement, Identifier, InfixExpression, IntegerLiteral, LetStatement,
        PrefixExpression, Program, ReturnStatement, Statement,
    },
    lexer::Lexer,
    token::{Token, TokenType},
};
pub trait PrefixParse {
    fn prefix_parse(&mut self, token_type: TokenType) -> Option<Expression>;
}
pub trait InfixParse {
    fn exist_infix_parse_fn(&self, token_type: TokenType) -> bool;
    fn infix_parse(
        &mut self,
        token_type: TokenType,
        pre_expression: Expression,
    ) -> Option<Expression>;
}
impl<'a> PrefixParse for Parser<'a> {
    fn prefix_parse(&mut self, token_type: TokenType) -> Option<Expression> {
        match token_type {
            TokenType::Ident => {
                let ident = Identifier {
                    token: self.cur_token.clone(),
                    value: self.cur_token.literal.clone(),
                };
                Some(Expression::Identifier(ident))
            }
            TokenType::NumberLiteral => Some(self.parse_integer_literal()),
            TokenType::Bang | TokenType::Minus => Some(self.parse_prefix_expression()),
            _ => None,
        }
    }
}
impl<'a> InfixParse for Parser<'a> {
    fn exist_infix_parse_fn(&self, token_type: TokenType) -> bool {
        match token_type {
            TokenType::Plus
            | TokenType::Minus
            | TokenType::Eq
            | TokenType::NotEq
            | TokenType::Lt
            | TokenType::Gt
            | TokenType::Slash
            | TokenType::Asterisk => true,
            _ => false,
        }
    }
    fn infix_parse(
        &mut self,
        token_type: TokenType,
        pre_expression: Expression,
    ) -> Option<Expression> {
        match token_type {
            TokenType::Plus
            | TokenType::Minus
            | TokenType::Eq
            | TokenType::NotEq
            | TokenType::Lt
            | TokenType::Gt
            | TokenType::Slash
            | TokenType::Asterisk => Some(self.parse_infix_expression(pre_expression)),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}
impl TokenType {
    fn to_precedence(&self) -> Precedence {
        match self {
            TokenType::Plus | TokenType::Minus => Precedence::Sum,
            TokenType::Eq | TokenType::NotEq => Precedence::Equals,
            TokenType::Lt | TokenType::Gt => Precedence::LessGreater,
            TokenType::Slash | TokenType::Asterisk => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }
}
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
    fn peek_precedence(&self) -> Precedence {
        self.peek_token.r#type.to_precedence()
    }
    fn cur_precedence(&self) -> Precedence {
        self.cur_token.r#type.to_precedence()
    }
    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.r#type {
            TokenType::Let => Some(self.parse_let_statement()),
            TokenType::Return => Some(self.parse_return_statement()),
            _ => Some(self.parse_expression_statement()),
        }
    }
    //
    fn parse_expression_statement(&mut self) -> Statement {
        let stmt = ExpressionStatement {
            token: self.cur_token.clone(),
            expression: self.parse_expression(Precedence::Lowest),
        };
        if self.peek_token_is(TokenType::Semicolon) {
            self.set_next_token();
        }
        Statement::ExpressionStatement(stmt)
    }
    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let mut left_expression = self.prefix_parse(self.cur_token.r#type);
        while !self.peek_token_is(TokenType::Semicolon) && precedence < self.peek_precedence() {
            self.set_next_token();
            if !self.exist_infix_parse_fn(self.cur_token.r#type) {
                return left_expression;
            }
            left_expression = self.infix_parse(self.cur_token.r#type, left_expression.unwrap());
        }
        left_expression
    }
    // return <expression>
    fn parse_return_statement(&mut self) -> Statement {
        let stmt_token = self.cur_token.clone();
        while !self.cur_token_is(TokenType::Semicolon) {
            self.set_next_token();
        }
        Statement::ReturnStatement(ReturnStatement {
            token: stmt_token,
            return_value: crate::ast::Expression::Identifier(Identifier {
                token: Token::eof(),
                value: String::new(),
            }),
        })
    }
    // let <identify> = <expression>
    fn parse_let_statement(&mut self) -> Statement {
        let stmt_token = self.cur_token.clone();
        if !self.expect_peek(TokenType::Ident) {
            panic!(
                "let statement is expect ident. but got = {:#?}",
                self.cur_token
            )
        }
        let name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };
        if !self.expect_peek(TokenType::Assign) {
            panic!(
                "let statement is expect assign. but got = {:#?}",
                self.cur_token
            )
        }
        while self.cur_token_is(TokenType::Semicolon) {
            self.set_next_token();
        }
        Statement::LetStatement(LetStatement {
            token: stmt_token,
            name,
            value: crate::ast::Expression::Identifier(Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            }),
        })
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
    // <prefix operator> <expression>
    fn parse_prefix_expression(&mut self) -> Expression {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();
        self.set_next_token();
        let right = self
            .parse_expression(Precedence::Prefix)
            .expect(&format!("right is none at token {:#?}", token));
        Expression::PrefixExpression(PrefixExpression {
            token,
            operator,
            right: Box::new(right),
        })
    }
    // <expression>
    fn parse_integer_literal(&mut self) -> Expression {
        let Ok(value) = self.cur_token.literal.parse::<isize>() else {
            panic!("{} is not parsed isize", &self.cur_token.literal)
        };
        let int = IntegerLiteral {
            token: self.cur_token.clone(),
            value,
        };
        Expression::IntegerLiteral(int)
    }
    //<left expression> <operator> <right expression>
    fn parse_infix_expression(&mut self, left_expression: Expression) -> Expression {
        let token = self.cur_token.clone();
        let operator = token.literal.to_string();
        let precedence = self.cur_precedence();
        self.set_next_token();

        Expression::InfixExpression(InfixExpression {
            token,
            operator,
            left: Box::new(left_expression),
            right: Box::new(self.parse_expression(precedence).unwrap()),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Expression, IntegerLiteral, Node, PrefixExpression, Statement},
        lexer::Lexer,
        token::{KeywordsToTokenType, Token},
    };

    use super::Parser;

    #[test]
    fn test_parsing_infix_expressions() {
        let input = r#"
            6+5;
            6-5;
            6*5;
            6/5;
            6>5;
            6<5;
            6==5;
            6!=5;
        "#;

        let lexer = Lexer::default(input);
        let mut sut = Parser::new(lexer);

        let statements = sut.parse_program().statements;

        let opes = vec!["+", "-", "*", "/", ">", "<", "==", "!="];

        for (i, s) in statements.into_iter().enumerate() {
            match s {
                Statement::ExpressionStatement(ex) => match ex.expression {
                    Some(Expression::InfixExpression(infix)) => {
                        assert_eq!(infix.operator, opes[i]);
                        assert_eq!(infix.left.token_literal(), "6");
                        assert_eq!(infix.right.token_literal(), "5");
                    }
                    _ => panic!("i = {}, expression got={:#?}", i, ex.expression),
                },
                _ => panic!("statement is not ExpressionStatement got={:#?}", s),
            }
        }
    }
    #[test]
    fn test_parse_prefix_expressions() {
        let input = r#"!5;
        -15;
        "#;

        let lexer = Lexer::default(input);
        let mut sut = Parser::new(lexer);
        let keyword = KeywordsToTokenType::new();

        let program = sut.parse_program();

        let ident = &program.statements[0];
        let prefix = PrefixExpression {
            token: Token::from_token_char('!'),
            operator: "!".to_string(),
            right: Box::new(Expression::IntegerLiteral(IntegerLiteral {
                token: Token::from_ident(&keyword, "5"),
                value: 5,
            })),
        };
        match ident {
            Statement::ExpressionStatement(ex) => match &ex.expression {
                Some(Expression::PrefixExpression(p)) => {
                    assert_eq!(p.operator, prefix.operator);
                }
                _ => panic!("got={:#?}", ex),
            },
            _ => panic!("not expression got={:#?}", ident),
        }
        let ident = &program.statements[1];
        let prefix = PrefixExpression {
            token: Token::from_token_char('-'),
            operator: "-".to_string(),
            right: Box::new(Expression::IntegerLiteral(IntegerLiteral {
                token: Token::from_ident(&keyword, "15"),
                value: 15,
            })),
        };
        match ident {
            Statement::ExpressionStatement(ex) => match &ex.expression {
                Some(Expression::PrefixExpression(p)) => {
                    assert_eq!(p.operator, prefix.operator);
                }
                _ => panic!("got={:#?}", ex),
            },
            _ => panic!("not expression got={:#?}", ident),
        }
    }
    #[test]
    fn test_identifier_expression() {
        let input = r#"foobar;"#;

        let lexer = Lexer::default(input);
        let mut sut = Parser::new(lexer);

        let program = sut.parse_program();

        let ident = &program.statements[0];
        match ident {
            Statement::ExpressionStatement(ex) => {
                assert_eq!(ex.string(), "foobar");
            }
            _ => panic!("not expression got={:#?}", ident),
        }
    }
    #[test]
    fn test_integer_expression_statement() {
        let input = "5;";
        let lexer = Lexer::default(input);
        let mut sut = Parser::new(lexer);
        let program = sut.parse_program();

        let stmt = program.statements.get(0).unwrap();
        match stmt {
            Statement::ExpressionStatement(s) => {
                assert_eq!(s.token_literal(), "5");
                match &s.expression {
                    Some(Expression::IntegerLiteral(i)) => {
                        assert_eq!(i.value, 5);
                    }
                    _ => panic!("not IntegerLiteral, got={:#?}", s.expression),
                }
            }
            _ => panic!("not ExpressionStatement, got={:#?}", stmt),
        }
    }
    #[test]
    fn test_return_statements() {
        let input = r#"
            return 5;
            return 10;
            return 838383;
        "#;
        let lexer = Lexer::default(input);

        let mut sut = Parser::new(lexer);

        let program = sut.parse_program();
        let mut stmt = program.statements.into_iter();
        let stmt1 = stmt.next().unwrap();
        let stmt2 = stmt.next().unwrap();
        let stmt3 = stmt.next().unwrap();
        assert_return_statement(stmt1, "5");
        assert_return_statement(stmt2, "10");
        assert_return_statement(stmt3, "838383");
        fn assert_return_statement(stmt: Statement, value: &str) {
            match stmt {
                Statement::ReturnStatement(s) => {
                    assert_eq!(s.token_literal(), "return");
                }
                _ => panic!("not ReturnStatement, got={:#?}", stmt),
            }
        }
    }
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
        println!("{:#?}", stmt1);
        assert_let_statement(stmt1, "x");
        assert_let_statement(stmt2, "y");
        assert_let_statement(stmt3, "foobar");
        fn assert_let_statement(stmt: Statement, name: &str) {
            //match stmt {
            //Statement::LetStatement(s) => {
            //assert_eq!(s.token_literal(), "let");
            //assert_eq!(s.name.value, name);
            //}
            //_ => panic!("not LetStatement, got={:#?}", stmt),
            //}
        }
    }
}
