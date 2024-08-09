use osta_lexer::token::{Token, TokenKind};

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedEOF,
    UnexpectedToken { found: Token, expected: &'static [TokenKind] },
    UnexpectedSymbol(usize),
}
