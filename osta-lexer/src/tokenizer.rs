use logos::{Lexer, Logos};

use crate::token::*;

#[derive(Debug, Clone)]
pub struct Tokenizer<'source> {
    lexer: Lexer<'source, TokenKind>,
    token: Option<Result<Token, usize>>
}

impl<'source> Tokenizer<'source> {
    pub fn new(input: &str) -> Tokenizer {
        let mut tokenizer = Tokenizer {
            lexer: TokenKind::lexer(input),
            token: None
        };
        tokenizer.next();
        tokenizer
    }

    /// Advances the iterator and returns the next value.
    ///
    /// Returns [`None`] when iteration is finished. Individual iterator
    /// implementations may choose to resume iteration, and so calling `next()`
    /// again may or may not eventually start returning [`Some(Item)`] again at some
    /// point.
    ///
    /// [`Some(Item)`]: Some
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<Result<Token, usize>> {
        let result = self.lexer.next().map(|r| r.map(|kind| Token {
            kind,
            span: self.lexer.span()
        })).map(|mut result| result.map_err(|_| self.lexer.span().start));
        self.token = result.clone();
        result
    }

    /// Returns the last token that was returned by `next` without advancing the iterator.
    pub fn peek(&self) -> Option<Result<Token, usize>> {
        self.token.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier() {
        let mut tokenizer = Tokenizer::new("hello @");
        assert_eq!(tokenizer.peek(), Some(Ok(Token {
            kind: TokenKind::Identifier,
            span: (0..5)
        })));
        assert_eq!(tokenizer.next(), Some(Err(6)));
    }
}
