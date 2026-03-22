// EcmaScript-262 notes
// 13.2.3 Literals
// 12.9.4 String Literals
#[derive(Debug, PartialEq, Clone)]

pub enum Token {
    Literal(Literal),
    Identifier(String),
    Keyword(Keyword),
    Operator(Operator),
    Punctuation(Punctuation),
    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    JNumber(f64),
    JBoolean(bool),
    JString(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Let,
    If,
    Else,
    While,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Punctuation {
    Semicolon,
    LBrace,
    RBrace,
    LParen,
    RParen,
    Colon,
    Comma,
    Dot,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Assign,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Not,
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    In,
}
