use osta_ast::{AstBuilder, NodeRef};
use osta_lexer::token::{Token, TokenKind};
use osta_lexer::tokenizer::Tokenizer;

use crate::error::ParseError;
use crate::fallible;

fn acquire(tokenizer: &mut Tokenizer) -> Result<Token, ParseError> {
    tokenizer.next()
        .ok_or(ParseError::UnexpectedEOF)?
        .map_err(|pos| ParseError::UnexpectedSymbol(pos))
}

fn tokenize(tokenizer: &mut Tokenizer, kind: &'static [TokenKind]) -> Result<Token, ParseError> {
    let token = acquire(tokenizer)?;
    if kind.contains(&token.kind) {
        Ok(token)
    } else {
        Err(ParseError::UnexpectedToken {
            found: token,
            expected: kind
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

pub fn parse_term(tokenizer: &mut Tokenizer, builder: &mut AstBuilder) -> Result<NodeRef, ParseError> {
    let token = acquire(tokenizer)?;
    match token.kind {
        TokenKind::Integer => parse_integer(tokenizer, builder),
        TokenKind::Identifier => parse_identifier(tokenizer, builder),
        TokenKind::LParen => {
            tokenizer.next();
            let expr = fallible!(parse_expression, tokenizer, builder)?;
            tokenize(tokenizer, &[TokenKind::RParen])?;
            Ok(expr)
        },
        TokenKind::Minus | TokenKind::Bang => {
            tokenizer.next();
            let expr = parse_term(tokenizer, builder)?;
            Ok(builder.push_unary(token, expr))
        },
        _ => Err(ParseError::UnexpectedToken { found: token, expected: &[
            TokenKind::Integer, TokenKind::Identifier, TokenKind::LParen, TokenKind::Minus, TokenKind::Bang
        ] })
    }
}

pub fn parse_expression(tokenizer: &mut Tokenizer, builder: &mut AstBuilder) -> Result<NodeRef, ParseError> {
    builder.checkpoint();
    let left = parse_term(tokenizer, builder)?;
    let op = tokenize(tokenizer, &[TokenKind::Plus, TokenKind::Minus, TokenKind::Asterisk])?;
    let right = parse_term(tokenizer, builder)?;
    builder.commit();
    Ok(builder.push_bin_expr(left, op, right))
}
