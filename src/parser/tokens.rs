#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Init,
    If,
    Elif,
    Else,
    For,
    While,
    In,
    Try,
    Catch,
    Finally,
    Throw,
    Class,
    Extends,
    Fixed,
    Secure,
    Return,
    Range,
    Log,
    Get,
    From,

    // Symbols
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Semicolon,
    Comma,
    Dot,
    Arrow,        // =>
    Star,         // *
    Equal,
    DoubleEqual,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Plus,
    Minus,
    Slash,
    Percent,
    Question,
    TernaryColon,
    Ellipsis,     // ...

    // Literals
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(String),
    BooleanLiteral(bool),
    TemplateString(String),
    Null,
    Undefined,

    // Comments
    Comment(String),

    // End of file
    EOF,
    Unknown(char),
}


#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}
