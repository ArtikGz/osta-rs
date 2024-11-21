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
            let result = $func(&mut lexer, $builder$(, $arg)*).map_err(|err| {
                $builder.rollback(&err).expect("rollback failed");
                err
            });
            if result.is_ok() {
                *$tokenizer = lexer;
                $builder.commit();
            }
            result
        }
    };
}
pub(crate) use fallible;

macro_rules! optional {
    ($func:expr, $tokenizer:expr, $builder:expr$(, $arg:expr)*) => {
        unsafe { fallible!($func, $tokenizer, $builder$(, $arg)*).or(Ok::<NodeRef, ParseError>(NodeRef::NULL)).unwrap_unchecked() }
    };
}
pub(crate) use optional;
