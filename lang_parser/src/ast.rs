use crate::token::Token;
#[derive(Debug)]
pub struct Program {
    statements: Vec<Statement>,
}

//impl Program {
//    pub fn new()
//}

pub trait Node {
    fn token_literal(&self) -> &str;
    fn string(&self) -> String;
}

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
}
impl Node for Statement {
    fn string(&self) -> String {
        String::new()
    }
    fn token_literal(&self) -> &str {
        ""
    }
}
#[derive(Debug)]
pub enum Expression {}
impl Node for Expression {
    fn string(&self) -> String {
        String::new()
    }
    fn token_literal(&self) -> &str {
        ""
    }
}

#[derive(Debug)]
pub struct LetStatement {
    token: Token,
    name: Identifier,
    expression: Expression,
}

#[derive(Debug)]
pub struct Identifier {
    token: Token,
    value: String,
}

macro_rules! impl_simple_node_trait {
    ($($node:ident),*) => {
        $(
            impl Node for $node {
                fn string(&self) -> String {
                    self.token.literal.clone()
                }
                fn token_literal(&self) -> &str {
                    &self.token.literal
                }
            }
        )*
    };
    ($($node:ident),*,) => {
       impl_simple_node_trait!($($node),*)
    };
}

impl_simple_node_trait!(Identifier, LetStatement);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn 簡単なnode_traitの実装はマクロで簡潔にできる() {
        let l = Identifier {
            token: Token::eof(),
            value: "EOF".to_string(),
        };
        assert_eq!(l.token_literal(), "");
        assert_eq!(l.string(), "");
    }
}
