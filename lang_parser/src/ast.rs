use crate::token::Token;
#[derive(Debug)]
pub struct Program {
    pub(super) statements: Vec<Statement>,
}

pub trait Node {
    fn token_literal(&self) -> &str;
    fn string(&self) -> String;
}

#[derive(Debug)]
pub enum Expression {
    L,
}
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
    pub(super) token: Token,
    pub(super) name: Identifier,
    pub(super) expression: Expression,
}
#[derive(Debug)]
pub struct ReturnStatement {
    pub(super) token: Token,
    pub(super) return_value: Expression,
}

#[derive(Debug)]
pub struct Identifier {
    pub(super) token: Token,
    pub(super) value: String,
}

macro_rules! declare_statement {
    ($($statement:ident),*) => {
        #[derive(Debug)]
        pub enum Statement {
            $($statement($statement),)*
        }
    };
}
declare_statement!(LetStatement, ReturnStatement);
macro_rules! impl_node_trait_for_statement {
    ($($statement:ident),*) => {
       impl Node for Statement {
            fn string(&self)->String {
                match self {
                    $(
                        Self::$statement(s)=>s.string(),
                    )*
                }
            }
            fn token_literal(&self)->&str {
                match self {
                    $(
                        Self::$statement(s)=>s.token_literal(),
                    )*
                }
            }
       }
    };
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

impl_node_trait_for_statement!(LetStatement, ReturnStatement);
impl_simple_node_trait!(Identifier, LetStatement, ReturnStatement);

#[cfg(test)]
mod tests {
    use crate::token::KeywordsToTokenType;

    use super::*;
    #[test]
    fn statementsのnode_traitの実装はマクロで簡潔にできる() {
        let key = KeywordsToTokenType::new();
        let name = Identifier {
            token: Token::from_ident(&key, "name"),
            value: "3".to_string(),
        };
        let l = Statement::LetStatement(LetStatement {
            token: Token::from_ident(&key, "let"),
            name: name,
            expression: Expression::L,
        });
        assert_eq!(l.string(), "let");
    }
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
