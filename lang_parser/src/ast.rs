use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> &str;
    fn string(&self) -> String;
}
pub trait NodeDebug {
    fn debug(&self);
}

pub trait Statement: Node {
    fn statement_node(&mut self);
}
pub trait Expression: Node + NodeDebug {
    fn expression_node(&mut self);
}

pub struct LetStatement {
    token: Token,
    name: Identifier,
    value: Box<dyn Expression>,
}

#[derive(Debug)]
pub struct Identifier {
    token: Token,
    value: String,
}
