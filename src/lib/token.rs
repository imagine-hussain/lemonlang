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
    PlusEq,
    Minus,
    MinusEq,
    Mult,
    MultEq,
    Div,
    DivEq,
    Not,
    NotEq,
    Eq,
    EqEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    And,
    AndEq,
    AndAnd,
    Or,
    OrEq,
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
    Colon,

    // Identifier
    Id(String),

    // literals
    IntLiteral(String),
    FloatLiteral(String),
    BooleanLiteral(bool),
    StringLiteral(String),

    // TODO: Should have associated Error Variants?
    Error,
}
