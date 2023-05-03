#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    // Keywords
    Bool,
    Break,
    Continue,
    Char,
    Else,
    Float,
    For,
    If,
    Int,
    Return,
    Struct,
    Enum,
    Void,
    While,
    Loop,
    Let,
    Const,
    Match,

    // operators
    Plus,
    PlusEq,
    Minus,
    MinusEq,
    Mult,
    MultEq,
    Div,
    DivEq,
    Dot,
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
    ColonColon,

    // Identifier
    Id(String),

    // literals
    IntLiteral(String),
    FloatLiteral(String),
    BooleanLiteral(bool),
    StringLiteral(String),
    CharLiteral(char),

    // TODO: Should have associated Error Variants?
    Error,
}
