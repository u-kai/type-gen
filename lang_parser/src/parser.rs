use crate::{ast::Program, lexer::Lexer, token::Token};

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
        Program {
            statements: Vec::new(),
        }
    }
    fn set_next_token(&mut self) {
        std::mem::swap(&mut self.peek_token, &mut self.cur_token);
        self.peek_token = self.l.next_token();
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

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

        for s in program.statements {
            //assert_eq!(s)
        }
    }
}
