pub use parser::*;

pub mod error;
pub mod rules;
pub mod parser;

macro_rules! fallible {
    ($func:expr, $tokenizer:expr, $builder:expr $(, $arg:expr)*) => {
        $func($tokenizer, $builder, $($arg),*).map_err(|err| {
            $builder.rollback(&err).expect("rollback failed");
            err
        })
    };
    ($func:expr, $tokenizer:expr, $builder:expr) => {
        $func($tokenizer, $builder).map_err(|err| {
            $builder.rollback(&err).expect("rollback failed");
            err
        })
    };
}
pub(crate) use fallible;
