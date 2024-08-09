pub use error::*;
pub use parser::*;
pub use rules::*;

pub mod error;
pub mod rules;
pub mod parser;

macro_rules! fallible {
    ($func:expr, $tokenizer:expr, $builder:expr $(, $arg:expr)*) => {
        {
            $builder.checkpoint();
            let defer = $func($tokenizer, $builder, $($arg),*).map_err(|err| {
                $builder.rollback(&err).expect("rollback failed");
                err
            });
            $builder.commit();
            defer
        }
    };
    ($func:expr, $tokenizer:expr, $builder:expr) => {
        {
            $builder.checkpoint();
            let defer = $func($tokenizer, $builder).map_err(|err| {
                $builder.rollback(&err).expect("rollback failed");
                err
            });
            $builder.commit();
            defer
        }
    };
}
pub(crate) use fallible;

macro_rules! optional {
    ($func:expr $(, $arg:expr)*) => {
        fallible!($func, $($arg),*).or(Ok(NodeRef::NULL))
    };
}
pub(crate) use optional;
