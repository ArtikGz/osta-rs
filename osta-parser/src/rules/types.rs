use osta_ast::{AstBuilder, Node, NodeRef};
use osta_lexer::{Token, TokenKind, Tokenizer};
use crate::{fallible, optional, parse_identifier, peek, tokenize, ParseError};
use crate::expr::parse_expression;

pub fn parse_type(tokenizer: &mut Tokenizer, builder: &mut AstBuilder) -> Result<NodeRef, ParseError> {
    if let Ok(left) = fallible!(parse_derived_type, tokenizer, builder) {
        if let Ok(_) = tokenize(tokenizer, &[TokenKind::Bang]) {
            let right = parse_type(tokenizer, builder)?;

            return Ok(builder.push_error_type(left, right))
        }

        return Ok(left);
    }

    parse_void(tokenizer, builder)
}

pub fn parse_void(tokenizer: &mut Tokenizer, builder: &mut AstBuilder) -> Result<NodeRef, ParseError> {
    tokenize(tokenizer, &[TokenKind::Void])?;

    Ok(builder.push_type(NodeRef::NULL))
}

pub fn parse_base_type(tokenizer: &mut Tokenizer, builder: &mut AstBuilder) -> Result<NodeRef, ParseError> {
    if let Ok(type_node) = fallible!(parse_void, tokenizer, builder) {
        let token = tokenize(tokenizer, &[TokenKind::Asterisk])?;

        return Ok(builder.push_type_modifier(type_node, token));
    }

    let node = parse_identifier(tokenizer, builder)?;
    Ok(builder.push_type(node))
}

// NOTE(ArtikGz): In the future, move parsing tuples into a metacompliation step in the stdlib
// creating a struct and implementing iterable + destructuring
pub fn parse_inner_tuple(tokenizer: &mut Tokenizer, builder: &mut AstBuilder) -> Result<NodeRef, ParseError> {
    let first = optional!(parse_derived_type, tokenizer, builder)?;
    if first == NodeRef::NULL {
        return Ok(NodeRef::NULL);
    }

    Ok(if let Ok(_) = tokenize(tokenizer, &[TokenKind::Comma]) {
        let next = parse_inner_tuple(tokenizer, builder)?;

        builder.push_tuple_type(first, next)
    } else {
        builder.push_tuple_type(first, NodeRef::NULL)
    })
}

pub fn parse_derived_type(tokenizer: &mut Tokenizer, builder: &mut AstBuilder) -> Result<NodeRef, ParseError> {
    if let Ok(_) = tokenize(tokenizer, &[TokenKind::LParen]) {
        let inner = parse_inner_tuple(tokenizer, builder)?;
        tokenize(tokenizer, &[TokenKind::RParen])?;

        // NOTE(ArtikGz): if tuple is (), inner would be NodeRef::NULL
        // so () type is equivalent to void type.
        return Ok(builder.push_type(inner));
    }

    let mut final_node = parse_base_type(tokenizer, builder)?;
    while let Ok(token) = peek(tokenizer) {
        match token.kind {
            TokenKind::LBracket => {
                let length = optional!(parse_expression, tokenizer, builder)?;
                tokenize(tokenizer, &[TokenKind::RBracket])?;

                final_node = builder.push_array_type(final_node, length);
            }
            TokenKind::Asterisk | TokenKind::QMark => {
                final_node = builder.push_type_modifier(final_node, token);
                tokenizer.next();
            }
            _ => {}
        }
    }

    Ok(final_node)
}

#[cfg(test)]
mod tests {
    use super::*;
    use osta_ast::{DataRef, Node, NodeKind};
    use crate::tests::{assert_ast, asterisk};

    #[test]
    fn test_void() {
        assert_ast!(parse_type, "void",
            [
                Node { kind: NodeKind::Type(NodeRef::NULL), .. },
            ],
            []
        );
    }

    #[test]
    fn test_void_pointer_pointer() {
        assert_ast!(parse_type, "void**",
            [
                Node { kind: NodeKind::Type(NodeRef::NULL), .. },
                Node { kind: NodeKind::TypeModifier { child_ref: NodeRef(0), modifier_ref: DataRef(0) }, .. },
                Node { kind: NodeKind::TypeModifier { child_ref: NodeRef(1), modifier_ref: DataRef(1) }, .. }
            ],
            [asterisk!(), asterisk!()]
        );
    }
}