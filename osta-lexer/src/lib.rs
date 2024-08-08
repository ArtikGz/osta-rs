pub mod error;
pub mod token;
pub mod tokens;

#[cfg(test)]
pub mod tests {
    macro_rules! lextest {
        ($name:ident, $func:ident, $input:expr, $expected:expr, $rest:expr) => {
            #[test]
            fn $name() {
                let mut tokenizer = Tokenizer::new($input);
                assert_eq!(tokenizer.$func(), $expected);
                assert_eq!(tokenizer.input, $rest);
            }
        };
    }
    pub(crate) use lextest;
}