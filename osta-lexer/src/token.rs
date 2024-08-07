#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub lexeme: &'a str,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Identifier,
    
    // Keywords
    While,
    If,
    Else,
    Return,
    Void,

    // Single character tokens
    Plus,
    Minus,
    Star,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Eq,
    Bang,
    QMark,
    Comma,
    Colon,

    // Multiple character tokens
    EqEq,
    BangEq,

    // Literals
    String,
    Int,

    // Other
    Eof,

    #[cfg(test)]
    Test,
}
