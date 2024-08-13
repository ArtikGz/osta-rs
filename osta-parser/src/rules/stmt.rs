use osta_ast::{AstBuilder, NodeRef};
use osta_lexer::*;
use crate::*;
use crate::expr::parse_expression;

pub fn parse_expr_stmt(tokenizer: &mut Tokenizer, builder: &mut AstBuilder) -> Result<NodeRef, ParseError> {
    let expr = parse_expression(tokenizer, builder)?;
    tokenize(tokenizer, &[TokenKind::Semicolon])?;
    Ok(builder.push_expr_stmt(expr))
}

pub fn parse_stmt(tokenizer: &mut Tokenizer, builder: &mut AstBuilder) -> Result<NodeRef, ParseError> {
    parse_expr_stmt(tokenizer, builder)
}

pub fn parse_stmt_in_block(tokenizer: &mut Tokenizer, builder: &mut AstBuilder) -> Result<NodeRef, ParseError> {
    let stmt = parse_stmt(tokenizer, builder)?;
    let next = optional!(parse_stmt_in_block, tokenizer, builder);
    Ok(builder.push_stmt(stmt, next))
}

pub fn parse_block(tokenizer: &mut Tokenizer, builder: &mut AstBuilder) -> Result<NodeRef, ParseError> {
    tokenize(tokenizer, &[TokenKind::LBrace])?;
    let stmt = optional!(parse_stmt_in_block, tokenizer, builder);
    tokenize(tokenizer, &[TokenKind::RBrace])?;
    Ok(builder.push_block(stmt))
}

#[cfg(test)]
mod tests {
    use super::*;
    use osta_ast::{DataRef, Node, NodeKind};
    use crate::tests::assert_ast;

    #[test]
    fn simple_block() {
        assert_ast!(parse_block, "{ 1; }",
        [
            Node { kind: NodeKind::IntegerLiteral(DataRef(0)), ..},
              _,
              _,
              Node { kind: NodeKind::Stmt { child_ref: NodeRef(2), next_ref: NodeRef::NULL},.. },
              Node { kind: NodeKind::Block { first_stmt_ref: NodeRef(3) }, .. }
        ],
        [
            ..,
        ]
    );
    }
}
