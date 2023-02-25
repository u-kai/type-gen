use crate::token::Token;
#[derive(Debug)]
pub struct Program {
    pub(super) statements: Vec<Statement>,
}
impl Program {
    pub fn string(&self) -> String {
        self.statements.iter().fold(String::new(), |acc, stmt| {
            format!("{}{}", acc, stmt.string())
        })
    }
}

pub trait Node {
    fn token_literal(&self) -> &str;
    fn string(&self) -> String;
}

#[derive(Debug)]
pub struct LetStatement {
    pub(super) token: Token,
    pub(super) name: Identifier,
    pub(super) value: Expression,
}
impl LetStatement {
    fn to_string(&self) -> String {
        format!(
            "{} {} = {};",
            self.token_literal(),
            self.name.string(),
            self.value.string()
        )
    }
}
#[derive(Debug)]
pub struct ReturnStatement {
    pub(super) token: Token,
    pub(super) return_value: Expression,
}
impl ReturnStatement {
    fn to_string(&self) -> String {
        format!("{} {};", self.token_literal(), self.return_value.string())
    }
}
#[derive(Debug)]
pub struct ExpressionStatement {
    pub(super) token: Token,
    pub(super) expression: Option<Expression>,
}
impl ExpressionStatement {
    fn to_string(&self) -> String {
        if let Some(ex) = self.expression.as_ref() {
            format!("{}", ex.string())
        } else {
            String::new()
        }
    }
}

#[derive(Debug)]
pub struct IntegerLiteral {
    pub(super) token: Token,
    pub(super) value: isize,
}
impl IntegerLiteral {
    fn to_string(&self) -> String {
        self.token.literal.clone()
    }
}
#[derive(Debug)]
pub struct Identifier {
    pub(super) token: Token,
    pub(super) value: String,
}
impl Identifier {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}
#[derive(Debug)]
pub struct PrefixExpression {
    pub(super) token: Token,
    pub(super) operator: String,
    pub(super) right: Box<Expression>,
}
impl PrefixExpression {
    fn to_string(&self) -> String {
        format!("({}{})", self.operator, self.right.string())
    }
}

macro_rules! declare_expression {
    ($($expression:ident),*) => {
        #[derive(Debug)]
        pub enum Expression {
            $($expression($expression),)*
        }
    };
}
macro_rules! declare_statement {
    ($($statement:ident),*) => {
        #[derive(Debug)]
        pub enum Statement {
            $($statement($statement),)*
        }
    };
}
macro_rules! impl_node_trait_for_expression {
    ($($expression:ident),*) => {
       impl Node for Expression {
            fn string(&self)->String {
                match self {
                    $(
                        Self::$expression(s)=>s.string(),
                    )*
                }
            }
            fn token_literal(&self)->&str {
                match self {
                    $(
                        Self::$expression(s)=>s.token_literal(),
                    )*
                }
            }
       }
    };
}
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
                    self.to_string()
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
declare_expression!(Identifier, IntegerLiteral, PrefixExpression);
impl_node_trait_for_expression!(Identifier, IntegerLiteral, PrefixExpression);
declare_statement!(LetStatement, ReturnStatement, ExpressionStatement);
impl_node_trait_for_statement!(LetStatement, ReturnStatement, ExpressionStatement);
impl_simple_node_trait!(
    Identifier,
    PrefixExpression,
    IntegerLiteral,
    LetStatement,
    ReturnStatement,
    ExpressionStatement
);

#[cfg(test)]
mod tests {
    use crate::token::KeywordsToTokenType;

    use super::*;
    #[test]
    fn statementsは文字列出力できる() {
        let keyword = KeywordsToTokenType::new();
        let program = Program {
            statements: vec![Statement::LetStatement(LetStatement {
                token: Token::from_ident(&keyword, "let"),
                name: Identifier {
                    token: Token::from_ident(&keyword, "myVar"),
                    value: "myVar".to_string(),
                },
                value: Expression::Identifier(Identifier {
                    token: Token::from_ident(&keyword, "anotherVar"),
                    value: "anotherVar".to_string(),
                }),
            })],
        };
        assert_eq!(program.string(), "let myVar = anotherVar;")
    }
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
            value: Expression::Identifier(Identifier {
                token: Token::eof(),
                value: "".to_string(),
            }),
        });
        assert_eq!(l.token_literal(), "let");
    }
    #[test]
    fn 簡単なnode_traitの実装はマクロで簡潔にできる() {
        let l = Identifier {
            token: Token::eof(),
            value: "EOF".to_string(),
        };
        assert_eq!(l.token_literal(), "");
    }
}
