pub use error::*;
pub use parser::*;
pub use rules::*;

pub mod error;
pub mod parser;
pub mod rules;

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
