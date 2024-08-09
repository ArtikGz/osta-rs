pub use error::*;
pub use parser::*;
pub use rules::*;

pub mod error;
pub mod rules;
pub mod parser;

macro_rules! fallible {
    ($func:expr, $tokenizer:expr, $builder:expr$(, $arg:expr)*) => {
        {
            $builder.checkpoint();
            let mut lexer = $tokenizer.clone();
            let defer = $func(&mut lexer, $builder$(, $arg)*).map_err(|err| {
                $builder.rollback(&err).expect("rollback failed");
                err
            });
            if defer.is_ok() {
                *$tokenizer = lexer;
                $builder.commit();
            }
            defer
        }
    };
}
pub(crate) use fallible;

macro_rules! optional {
    ($func:expr, $tokenizer:expr, $builder:expr$(, $arg:expr)*) => {
        fallible!($func, $tokenizer, $builder$(, $arg)*).or(Ok(NodeRef::NULL))
    };
}
pub(crate) use optional;

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

