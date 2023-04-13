#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Bool,
    Break,
    Continue,
    Else,
    Float,
    For,
    If,
    Int,
    Return,
    Void,
    While,

    // operators
    Plus,
    Minus,
    Mult,
    Div,
    Not,
    NotEq,
    Eq,
    EqEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    And,
    AndAnd,
    Or,
    OrOr,
    Stab,
    Arrow,

    // separators
    LCurly,
    RCurly,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Semicolon,
    Comma,

    // identifiers
    Id(String),

    // literals
    IntLiteral(String),
    FloatLiteral(String),
    BooleanLiteral(bool),
    StringLiteral(String),

    // special tokens...
    Error,
    Eof,
}
