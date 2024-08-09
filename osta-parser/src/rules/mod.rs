mod expr;

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

    macro_rules! assert_ast {
        ($p:expr, $input:expr, $nodes:pat, $datas:pat) => {{
            let (_, rest) = dbg!($p.apply($input));

            // NOTE(cdecompilador): using matches! here may make some literal matching painful
            // for example we can't write !0 inside
            assert!(matches!(
                &rest.builder.borrow().ast.nodes[..],
                $nodes
            ));
            assert!(matches!(
                &rest.builder.borrow().ast.datas[..],
                $datas
            ));

            rest
        }};
    }
    pub(crate) use assert_ast;
}
