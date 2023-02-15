#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Fn,
    Struct,
    StringLiteral,
    NumberLiteral,
    Comment,
    Eq,
    Eof,
}
