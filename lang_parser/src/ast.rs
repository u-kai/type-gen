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
pub struct BlockStatement {
    pub(super) token: Token,
    pub(super) statements: Vec<Statement>,
}
impl BlockStatement {
    fn to_string(&self) -> String {
        self.statements
            .iter()
            .fold(String::new(), |acc, cur| format!("{}{}", acc, cur.string()))
    }
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
    pub(super) return_value: Option<Expression>,
}
impl ReturnStatement {
    fn to_string(&self) -> String {
        let return_value_string = if let Some(return_value) = &self.return_value {
            return_value.string()
        } else {
            String::new()
        };
        format!("{} {};", self.token_literal(), return_value_string)
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
pub struct FunctionalLiteral {
    pub(super) token: Token,
    pub(super) parameters: Vec<Identifier>,
    pub(super) body: BlockStatement,
}
impl FunctionalLiteral {
    fn to_string(&self) -> String {
        let params = self.parameters.iter().fold(String::new(), |acc, cur| {
            format!("{}{},", acc, cur.string())
        });
        format!(
            "{} ({}) {}",
            self.token_literal(),
            params,
            self.body.string()
        )
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
// if (<condition>) {<consequence>} else {<alternative>}
#[derive(Debug)]
pub struct IfExpression {
    pub(super) token: Token,
    pub(super) condition: Box<Expression>,
    pub(super) consequence: BlockStatement,
    pub(super) alternative: Option<BlockStatement>,
}
impl IfExpression {
    fn to_string(&self) -> String {
        let alternative_string = if let Some(alt) = &self.alternative {
            format!(" else {}", alt.string())
        } else {
            String::new()
        };
        format!(
            "if {} {}{}",
            self.condition.string(),
            self.consequence.string(),
            alternative_string
        )
    }
}
#[derive(Debug)]
pub struct Boolean {
    pub(super) token: Token,
    pub(super) value: bool,
}
impl Boolean {
    fn to_string(&self) -> String {
        self.value.to_string()
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
#[derive(Debug)]
pub struct CallExpression {
    pub(super) token: Token,
    pub(super) function: Box<Expression>,
    pub(super) arguments: Vec<Expression>,
}
impl CallExpression {
    fn to_string(&self) -> String {
        let args = self.arguments.iter().fold(String::new(), |acc, cur| {
            format!("{}{},", acc, cur.string())
        });
        format!("{}({})", self.function.string(), args)
    }
}

//<left expression> <operator> <right expression>
#[derive(Debug)]
pub struct InfixExpression {
    pub(super) token: Token,
    pub(super) operator: String,
    pub(super) left: Box<Expression>,
    pub(super) right: Box<Expression>,
}
impl InfixExpression {
    fn to_string(&self) -> String {
        format!(
            "({} {} {})",
            self.left.string(),
            self.operator,
            self.right.string()
        )
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
declare_expression!(
    Identifier,
    IntegerLiteral,
    FunctionalLiteral,
    PrefixExpression,
    InfixExpression,
    IfExpression,
    CallExpression,
    Boolean
);
impl_node_trait_for_expression!(
    Identifier,
    CallExpression,
    IntegerLiteral,
    FunctionalLiteral,
    PrefixExpression,
    InfixExpression,
    IfExpression,
    Boolean
);
declare_statement!(
    LetStatement,
    ReturnStatement,
    ExpressionStatement,
    BlockStatement
);
impl_node_trait_for_statement!(
    LetStatement,
    ReturnStatement,
    ExpressionStatement,
    BlockStatement
);
impl_simple_node_trait!(
    Identifier,
    Boolean,
    FunctionalLiteral,
    IfExpression,
    InfixExpression,
    PrefixExpression,
    IntegerLiteral,
    LetStatement,
    ReturnStatement,
    BlockStatement,
    CallExpression,
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
