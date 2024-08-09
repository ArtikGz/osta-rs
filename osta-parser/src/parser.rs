use osta_ast::{AstBuilder, NodeRef};
use osta_lexer::token::{Token, TokenKind};
use osta_lexer::tokenizer::Tokenizer;

use crate::error::ParseError;
use crate::{fallible, optional};

fn peek(tokenizer: &mut Tokenizer) -> Result<Token, ParseError> {
    tokenizer.peek()
        .ok_or(ParseError::UnexpectedEOF)?
        .map_err(|pos| ParseError::UnexpectedSymbol(pos))
}

fn tokenize(tokenizer: &mut Tokenizer, kind: &'static [TokenKind]) -> Result<Token, ParseError> {
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

pub fn parse_function_call_args(tokenizer: &mut Tokenizer, builder: &mut AstBuilder) -> Result<NodeRef, ParseError> {
    let arg0 = parse_expression(tokenizer, builder)?;
    let next = optional!(parse_function_call_args, tokenizer, builder)?;
    Ok(builder.push_func_call_arg(arg0, next))
}

pub fn parse_function_call(tokenizer: &mut Tokenizer, builder: &mut AstBuilder) -> Result<NodeRef, ParseError> {
    let name = parse_identifier(tokenizer, builder)?;
    tokenize(tokenizer, &[TokenKind::LParen])?;
    builder.checkpoint();
    let args = optional!(parse_function_call_args, tokenizer, builder)?;
    builder.commit();
    tokenize(tokenizer, &[TokenKind::RParen])?;
    Ok(builder.push_func_call(name, args))
}

pub fn parse_term(tokenizer: &mut Tokenizer, builder: &mut AstBuilder) -> Result<NodeRef, ParseError> {
    if let Ok(node) = fallible!(parse_function_call, tokenizer, builder) {
        return Ok(node);
    }

    let token = peek(tokenizer)?;
    let node = match token.kind {
        TokenKind::Integer => parse_integer(tokenizer, builder)?,
        TokenKind::Identifier => parse_identifier(tokenizer, builder)?,
        TokenKind::LParen => {
            tokenizer.next();
            let expr = parse_expression(tokenizer, builder)?;
            tokenize(tokenizer, &[TokenKind::RParen])?;
            expr
        }
        TokenKind::Minus | TokenKind::Bang => {
            tokenizer.next();
            let expr = parse_expression(tokenizer, builder)?;
            builder.push_unary(token, expr)
        }
        _ => Err(ParseError::UnexpectedToken {
            found: token,
            expected: &[
                TokenKind::Integer, TokenKind::Identifier, TokenKind::LParen, TokenKind::Minus, TokenKind::Bang
            ],
        })?
    };
    Ok(builder.push_term(node))
}

pub fn parse_expression(tokenizer: &mut Tokenizer, builder: &mut AstBuilder) -> Result<NodeRef, ParseError> {
    if let Ok(node) = fallible!(parse_term, tokenizer, builder) {
        return Ok(node);
    } // TODO: Handle error accumulation

    let left = parse_term(tokenizer, builder)?;
    let op = tokenize(tokenizer, &[TokenKind::Plus, TokenKind::Minus, TokenKind::Asterisk])?;
    let right = parse_term(tokenizer, builder)?;
    Ok(builder.push_bin_expr(left, op, right))
}
