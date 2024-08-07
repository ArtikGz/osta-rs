use osta_func::defer;
use osta_lexer::tokens;

use crate::Parser;
use crate::rules::*;

pub fn base_type<'a>() -> impl Parser<'a> {
    (do_parse! {
        from_emitter(tokens::kw_void());
        with_builder(move |mut builder| {
            builder.push_type(NodeRef::NULL)
        });
    }).or(do_parse! {
        token_ref = from_emitter(tokens::identifier());
        with_builder(move |mut builder| {
            let child_ref = builder.push_identifier(token_ref);
            builder.push_type(child_ref)
        });
    })
}

pub fn derived_type<'a>() -> impl Parser<'a> {
    base_type().or(do_parse! {
        type_ref = defer!(derived_type());
        token_ref = from_emitter(tokens::star());
        with_builder(move |mut builder| {
            builder.push_type_modifier(type_ref, token_ref)
        });
    }).or(do_parse! {
        type_ref = defer!(derived_type());
        from_emitter(tokens::lbracket());
        num_ref = defer!(expr()).optional();
        from_emitter(tokens::rbracket());
        with_builder(move |mut builder| {
            builder.push_array_type(type_ref, num_ref)
        });
    }).or(do_parse! {
        from_emitter(tokens::lparen());
        type_ref = defer!(derived_type());
        from_emitter(tokens::rparen());
        with_builder(move |_| {
            type_ref
        });
    }).or(do_parse! {
        type_ref = defer!(derived_type());
        token_ref = from_emitter(tokens::qmark());
        with_builder(move |mut builder| {
            builder.push_type_modifier(type_ref, token_ref)
        });
    }).or(do_parse! {
        type_ref = defer!(derived_type());
        from_emitter(tokens::bang());
        err_ref = defer!(derived_type());
        with_builder(move |mut builder| {
            builder.push_error_type(type_ref, err_ref)
        });
    })
}

fn params<'a>() -> impl Parser<'a> {
    do_parse! {
        type_ref = derived_type();
        name_ref = from_emitter(tokens::identifier());
        next_ref = do_parse! {
            from_emitter(tokens::comma());
            defer!(params());
        }.optional();
        with_builder(move |mut builder| {
            let name_ref = builder.push_identifier(name_ref);
            builder.push_param_decl(type_ref, name_ref, next_ref)
        });
    }
}

pub fn function_decl<'a>() -> impl Parser<'a> {
    do_parse! {
        type_ref = derived_type();
        name_ref = from_emitter(tokens::identifier());
        from_emitter(tokens::lparen());
        params_ref = defer!(params());
        from_emitter(tokens::rparen());
        with_builder(move |mut builder| {
            let name_ref = builder.push_identifier(name_ref);
            builder.push_func_decl(type_ref, name_ref, params_ref)
        });
    }
}

pub fn function_def<'a>() -> impl Parser<'a> {
    do_parse! {
        decl_ref = function_decl();
        body_ref = defer!(block());
        with_builder(move |mut builder| {
            builder.push_func_def(decl_ref, body_ref)
        });
    }
}

#[cfg(test)]
mod tests {
    use osta_ast::*;
    use osta_func::*;
    use osta_lexer::token::*;
    use crate::rules::tests::*;

    #[test]
    fn void_type() {
        let input = input!("void");
        assert_ast!(
            super::base_type(), input,
            [
                Node { kind: NodeKind::Type(NodeRef::NULL), parent_ref: NodeRef::NULL }
            ],
            [..]
        );
    }

    #[test]
    fn identifier_type() {
        let input = input!("Foo");
        assert_ast!(
            super::base_type(), input,
            [
                Node { kind: NodeKind::Identifier(_), parent_ref: NodeRef(1) },
                Node { kind: NodeKind::Type(NodeRef(0)), parent_ref: NodeRef::NULL }
            ],
            [..]
        );
    }

    #[test]
    fn pointer_type() {
        let input = input!("Foo*");
        assert_ast!(
            super::derived_type(), input,
            [
                Node { kind: NodeKind::Identifier(_), parent_ref: NodeRef(1) },
                Node { kind: NodeKind::Type(NodeRef(0)), parent_ref: NodeRef(2) },
                Node { kind: NodeKind::TypeModifier { child_ref: NodeRef(1), modifier_ref: DataRef(1) }, parent_ref: NodeRef::NULL }
            ],
            [
                _,
                Data::Token(Token { kind: TokenKind::Star, .. })
            ]
        );
    }

    #[test]
    fn array_type() {
        let input = input!("Foo[10]");
        assert_ast!(
            super::derived_type(), input,
            [
                Node { kind: NodeKind::Identifier(_), parent_ref: NodeRef(1) },
                Node { kind: NodeKind::Type(NodeRef(0)), parent_ref: NodeRef(2) },
                Node { kind: NodeKind::ArrayType { child_ref: NodeRef(1), length_ref: _ }, parent_ref: NodeRef::NULL }
            ],
            [..]
        );
    }

    #[test]
    fn paren_type() {
        let input = input!("(Foo)");
        assert_ast!(
            super::derived_type(), input,
            [
                Node { kind: NodeKind::Identifier(_), parent_ref: NodeRef(1) },
                Node { kind: NodeKind::Type(NodeRef(0)), parent_ref: NodeRef(2) }
            ],
            [..]
        );
    }

    #[test]
    fn optional_type() {
        let input = input!("Foo?");
        assert_ast!(
            super::derived_type(), input,
            [
                Node { kind: NodeKind::Identifier(_), parent_ref: NodeRef(1) },
                Node { kind: NodeKind::Type(NodeRef(0)), parent_ref: NodeRef(2) },
                Node { kind: NodeKind::TypeModifier{ child_ref: NodeRef(1), modifier_ref: DataRef(1) }, parent_ref: NodeRef::NULL }
            ],
            [
                _,
                Data::Token(Token { kind: TokenKind::QMark, .. })
            ]
        );
    }

    #[test]
    fn error_type() {
        let input = input!("Foo!Bar");
        assert_ast!(
            super::derived_type(), input,
            [
                Node { kind: NodeKind::Identifier(_), parent_ref: NodeRef(1) },
                Node { kind: NodeKind::Type(NodeRef(0)), parent_ref: NodeRef(4) },
                Node { kind: NodeKind::Identifier(_), parent_ref: NodeRef(3) },
                Node { kind: NodeKind::Type(NodeRef(0)), parent_ref: NodeRef(4) },
                Node { kind: NodeKind::ErrorType { child_ref: NodeRef(1), error_ref: NodeRef(3) }, parent_ref: NodeRef::NULL }
            ],
            [..]
        );
    }
}