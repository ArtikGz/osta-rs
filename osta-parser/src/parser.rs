use osta_ast::{AstBuilder, NodeRef};
use osta_lexer::*;

use crate::error::ParseError;
pub use crate::{fallible, optional};

pub fn peek(tokenizer: &mut Tokenizer) -> Result<Token, ParseError> {
    tokenizer.peek()
        .ok_or(ParseError::UnexpectedEOF)?
        .map_err(|pos| ParseError::UnexpectedSymbol(pos))
}

pub fn tokenize(tokenizer: &mut Tokenizer, kind: &'static [TokenKind]) -> Result<Token, ParseError> {
    let token = peek(tokenizer)?;
    if kind.contains(&token.kind) {
        tokenizer.next();
        Ok(token)
    } else {
        Err(ParseError::UnexpectedToken {
            found: token,
            expected: kind,
        })
    }
}

pub fn parse_integer(tokenizer: &mut Tokenizer, builder: &mut AstBuilder) -> Result<NodeRef, ParseError> {
    tokenize(tokenizer, &[TokenKind::Integer]).map(|token| {
        builder.push_integer(token)
    })
}

pub fn parse_identifier(tokenizer: &mut Tokenizer, builder: &mut AstBuilder) -> Result<NodeRef, ParseError> {
    tokenize(tokenizer, &[TokenKind::Identifier]).map(|token| {
        builder.push_identifier(token)
    })
}
