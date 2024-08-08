use logos::{Logos, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Logos, Debug, Copy, Clone, PartialEq, Eq)]
#[logos(skip r#"[\s\t\n\r\f]+"#)]
pub enum TokenKind {
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    
    // Keywords
    #[token("while")]
    While,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("return")]
    Return,
    #[token("void")]
    Void,

    // Single character tokens
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token(";")]
    Semicolon,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("=")]
    Eq,
    #[token("!")]
    Bang,
    #[token("?")]
    QMark,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,

    // Literals
    #[regex(r#""([^"]|\")*""#)]
    String,
    #[regex("[0-9]+")]
    Int,
}
