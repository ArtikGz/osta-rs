pub mod expr;
mod stmt;
mod flow;

#[cfg(test)]
pub mod tests {
    macro_rules! input {
        ($str_input:expr) => {{
            crate::ParserInput {
                input: $str_input,
                builder: std::rc::Rc::new(std::cell::RefCell::new(osta_ast::AstBuilder::new()))
            }
        }};
    }
    pub(crate) use input;

    #[cfg(test)]
    pub mod tests {
        macro_rules! assert_ast {
            ($func:expr, $input:expr, $nodes:pat, $datas:pat) => {{
                let (mut tokenizer, mut ast_builder) = (Tokenizer::new($input), AstBuilder::new());
                let _result = $func(&mut tokenizer, &mut ast_builder).unwrap();

                dbg!(&ast_builder.ast);

                // NOTE(cdecompilador): using matches! here may make some literal matching painful
                // for example we can't write !0 inside
                assert!(matches!(
                    ast_builder.ast.nodes[..],
                    $nodes
                ));

                assert!(matches!(
                    ast_builder.ast.datas[..],
                    $datas
                ));
            }};
        }
        pub(crate) use assert_ast;
    }
}
