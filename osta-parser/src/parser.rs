use osta_ast::{AstBuilder, NodeRef};
use osta_lexer::token::{Token, TokenKind};
use osta_lexer::tokenizer::Tokenizer;

use crate::error::ParseError;

pub fn token(tokenizer: &mut Tokenizer, kind: TokenKind) -> Result<Token, ParseError> {
    let tok = tokenizer.peek().ok_or(ParseError::UnexpectedEOF)?;
    match tok {
        Ok(token) => if token.kind == TokenKind::Int {
            tokenizer.next();
            Ok(token)
        } else {
            Err(ParseError::UnexpectedToken { found: token, expected: kind })
        },
        Err(pos) => Err(ParseError::UnexpectedSymbol(pos))
    }
}

pub fn parse_int(tokenizer: &mut Tokenizer, builder: &mut AstBuilder) -> Result<NodeRef, ParseError> {
    token(tokenizer, TokenKind::Int).map(|token| {
        builder.push_integer(token)
    })
}
