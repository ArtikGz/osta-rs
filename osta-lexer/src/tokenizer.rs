use logos::{Lexer, Logos};

use crate::token::*;

#[derive(Debug, Clone)]
pub struct Tokenizer<'source> {
    lexer: Lexer<'source, TokenKind>,
    token: Option<Result<Token, ()>>
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
    pub fn next(&mut self) -> Option<Result<Token, ()>> {
        let result = self.lexer.next().map(|r| r.map(|kind| Token {
            kind,
            span: self.lexer.span()
        }));
        self.token = result.clone();
        result
    }

    /// Returns the last token that was returned by `next` without advancing the iterator.
    pub fn peek(&self) -> Option<Result<Token, ()>> {
        self.token.clone()
    }

    /// Returns the current token if it is of the specified kind and advances the iterator.
    pub fn check(&mut self, kind: TokenKind) -> Option<Token> {
        match self.peek()? {
            Ok(token) if token.kind == kind => {
                self.next();
                Some(token)
            },
            _ => None
        }
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
        assert_eq!(tokenizer.next(), Some(Err(())));
    }
}
