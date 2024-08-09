use osta_ast::{AstBuilder, NodeRef};
use osta_lexer::{TokenKind, Tokenizer};
use crate::{fallible, optional, tokenize, ParseError};
use crate::expr::parse_expression;

pub fn parse_if_stmt(tokenizer: &mut Tokenizer, builder: &mut AstBuilder) -> Result<NodeRef, ParseError> {
    tokenize(tokenizer, &[TokenKind::If])?;
    let cond = parse_expression(tokenizer, builder)?;
    let if_then = parse_expression(tokenizer, builder)?;
    let if_else = match tokenize(tokenizer, &[TokenKind::Else]) {
        Ok(_) => parse_expression(tokenizer, builder)?,
        Err(_) => NodeRef::NULL,
    };

    Ok(builder.push_if_stmt(cond, if_then, if_else))
}

pub fn parse_do_while_stmt(tokenizer: &mut Tokenizer, builder: &mut AstBuilder) -> Result<NodeRef, ParseError> {
    tokenize(tokenizer, &[TokenKind::Do])?;
    let expr = optional!(parse_expression, tokenizer, builder)?;
    tokenize(tokenizer, &[TokenKind::While])?;
    let cond = parse_expression(tokenizer, builder)?;

    Ok(builder.push_do_while(expr, cond))
}

#[cfg(test)]
mod tests {
    use super::*;
    use osta_ast::{DataRef, Node, NodeKind};
    use crate::tests::{assert_ast, int, identifier};


    #[test]
    fn simple_if_else_statement() {
        assert_ast!(parse_if_stmt, "if 1 1 else 0",
            [
                // cond
                Node { kind: NodeKind::IntegerLiteral(DataRef(0)), .. },
                Node { kind: NodeKind::Term(NodeRef(0)), .. },
                // if_then
                Node { kind: NodeKind::IntegerLiteral(DataRef(1)), .. },
                Node { kind: NodeKind::Term(NodeRef(2)), .. },
                // else
                Node { kind: NodeKind::IntegerLiteral(DataRef(2)), .. },
                Node { kind: NodeKind::Term(NodeRef(4)), .. },
                Node { kind: NodeKind::IfStmt { cond_ref: NodeRef(1), then_block_ref: NodeRef(3), else_block_ref: NodeRef(5)  }, .. }
            ],
            [int!(), int!(), int!()]
        );
    }

    #[test]
    fn if_statement_with_block_expression() {
        assert_ast!(parse_if_stmt, "if should_enter_if() { func1(); func2(); } ",
            [
                // cond
                Node { kind: NodeKind::Identifier(DataRef(0)), .. },
                Node { kind: NodeKind::FuncCallExpr { name_ref: NodeRef(0), first_param_ref: NodeRef::NULL }, .. },
                // if_then
                // func1()
                Node { kind: NodeKind::Identifier(DataRef(1)), .. },
                Node { kind: NodeKind::FuncCallExpr { name_ref: NodeRef(2), first_param_ref: NodeRef::NULL }, .. },
                Node { kind: NodeKind::ExprStmt { expr_ref: NodeRef(3) }, .. },
                // func2()
                Node { kind: NodeKind::Identifier(DataRef(2)), .. },
                Node { kind: NodeKind::FuncCallExpr { name_ref: NodeRef(5), first_param_ref: NodeRef::NULL }, .. },
                Node { kind: NodeKind::ExprStmt { expr_ref: NodeRef(6) }, .. },

                Node { kind: NodeKind::Stmt { child_ref: NodeRef(7), next_ref: NodeRef::NULL }, ..},
                Node { kind: NodeKind::Stmt { child_ref: NodeRef(4), next_ref: NodeRef(8) }, ..},
                Node { kind: NodeKind::Block { first_stmt_ref: NodeRef(9) }, .. },
                // if
                Node { kind: NodeKind::IfStmt { cond_ref: NodeRef(1), then_block_ref: NodeRef(10), else_block_ref: NodeRef::NULL  }, .. }
            ],
            [identifier!(), identifier!(), identifier!()]
        );
    }

    #[test]
    fn do_nobody_while_cond() {
        assert_ast!(parse_do_while_stmt, "do while {}",
            [
                Node { kind: NodeKind::Block { first_stmt_ref: NodeRef::NULL}, .. },
                Node { kind: NodeKind::DoWhile { do_expr_ref: NodeRef::NULL, cond_ref: NodeRef(0) }, .. }
            ],
            []
        );
    }
}