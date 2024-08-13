pub mod expr;
pub mod flow;
pub mod stmt;
pub mod types;

#[cfg(test)]
pub mod tests {

    // Helper macros for tests
    macro_rules! int {
        () => {
            osta_ast::Data::Token(osta_lexer::Token { kind: TokenKind::Integer, .. })
        };
    }
    pub(crate) use int;

    macro_rules! identifier {
        () => {
            osta_ast::Data::Token(osta_lexer::Token { kind: TokenKind::Identifier, .. })
        };
    }
    pub(crate) use identifier;

    macro_rules! asterisk {
        () => {
            osta_ast::Data::Token(osta_lexer::Token { kind: TokenKind::Asterisk, .. })
        };
    }
    pub(crate) use asterisk;

    macro_rules! assert_ast {
        ($func:expr, $input:expr, $nodes:pat, $datas:pat) => {{
            let (mut tokenizer, mut ast_builder) = (Tokenizer::new($input), AstBuilder::new());
            let _result = $func(&mut tokenizer, &mut ast_builder).unwrap();

            dbg!(&ast_builder.ast);

            // NOTE(cdecompilador): using matches! here may make some literal matching painful
            // for example we can't write !0 inside
            assert!(matches!(ast_builder.ast.nodes[..], $nodes));

            assert!(matches!(ast_builder.ast.datas[..], $datas));
        }};
    }
    pub(crate) use assert_ast;
}
