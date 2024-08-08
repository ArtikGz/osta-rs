use crate::error::*;
use crate::token::*;

lazy_static::lazy_static! {
    static ref RE_INT: regex::Regex = regex::Regex::new("^[0-9]+").unwrap();
    static ref RE_IDENTIFIER: regex::Regex = regex::Regex::new("^[_a-zA-Z][_a-zA-Z0-9]*").unwrap();
}

macro_rules! emitter {
    ($name:ident, $self:ident, $parser:expr) => {
        pub fn $name(&mut $self) -> Result<Token<'a>, TokenizerError> {
            $self.skip_whitespace();
            $parser
        }
    };
}

struct Tokenizer<'a> {
    input: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &str) -> Tokenizer {
        Tokenizer { input }
    }

    fn skip_whitespace(&mut self) {
        self.input = self.input.trim_start();
    }

    fn token(&mut self, expected: &'static str, kind: TokenKind) -> Result<Token<'a>, TokenizerError> {
        if let Some(rest) = self.input.strip_prefix(expected) {
            let token = Token {
                lexeme: expected,
                kind,
            };
            self.input = rest;
            Ok(token)
        } else {
            Err(TokenizerError {
                found: self.input,
                kind: TokenizerErrorKind::ExpectedLiteral(expected),
            })
        }
    }

    fn keyword(&mut self, kw: &'static str, kind: TokenKind) -> Result<Token<'a>, TokenizerError> {
        self.token(kw, kind).map_err(|err| TokenizerError {
            found: err.found,
            kind: TokenizerErrorKind::ExpectedKeyword(kw),
        })
    }

    fn regex(
        &mut self,
        re: &'static regex::Regex,
        kind: TokenKind,
    ) -> Result<Token<'a>, TokenizerError> {
        if let Some(matched) = re.find(self.input) {
            let token = Token {
                lexeme: &self.input[matched.start()..matched.end()],
                kind,
            };
            self.input = &self.input[matched.end()..];
            Ok(token)
        } else {
            Err(TokenizerError {
                found: self.input,
                kind: TokenizerErrorKind::ExpectedRegex(re.as_str()),
            })
        }
    }

    emitter!(kw_void, self, self.keyword("void", TokenKind::Void));
    emitter!(kw_while, self, self.keyword("while", TokenKind::While));
    emitter!(kw_if, self, self.keyword("if", TokenKind::If));
    emitter!(kw_else, self, self.keyword("else", TokenKind::Else));
    emitter!(kw_return, self, self.keyword("return", TokenKind::Return));
    emitter!(integer, self, self.regex(&RE_INT, TokenKind::Int));
    emitter!(identifier, self, self.regex(&RE_IDENTIFIER, TokenKind::Identifier).map_err(|err| TokenizerError {
        found: err.found,
        kind: TokenizerErrorKind::ExpectedIdentifier
    }));
    emitter!(lparen, self, self.token("(", TokenKind::LParen));
    emitter!(rparen, self, self.token(")", TokenKind::RParen));
    emitter!(lbrace, self, self.token("{", TokenKind::LBrace));
    emitter!(rbrace, self, self.token("}", TokenKind::RBrace));
    emitter!(lbracket, self, self.token("[", TokenKind::LBracket));
    emitter!(rbracket, self, self.token("]", TokenKind::RBracket));
    emitter!(plus, self, self.token("+", TokenKind::Plus));
    emitter!(minus, self, self.token("-", TokenKind::Minus));
    emitter!(star, self, self.token("*", TokenKind::Star));
    emitter!(semicolon, self, self.token(";", TokenKind::Semicolon));
    emitter!(colon, self, self.token(":", TokenKind::Colon));
    emitter!(comma, self, self.token(",", TokenKind::Comma));
    emitter!(eq, self, self.token("=", TokenKind::Eq));
    emitter!(bang, self, self.token("!", TokenKind::Bang));
    emitter!(qmark, self, self.token("?", TokenKind::QMark));
    emitter!(bin_op, self, match self.plus() {
        Ok(token) => Ok(token),
        Err(_) => match self.minus() {
            Ok(token) => Ok(token),
            Err(_) => self.star(), // TODO: handle all errors
        },
    });
    emitter!(unary_op, self, match self.minus() {
        Ok(token) => Ok(token),
        Err(_) => self.bang(), // TODO: handle all errors
    });
}

#[cfg(test)]
mod tests {
    use crate::tests::lextest;
    use super::*;

    lextest!(ok_kw_void, kw_void, "void", Ok(Token { lexeme: "void", kind: TokenKind::Void }), "");
    lextest!(extra_kw_void, kw_void, "void foo", Ok(Token { lexeme: "void", kind: TokenKind::Void }), " foo");
    lextest!(err_kw_void, kw_void, "foo", Err(TokenizerError {
        found: "foo",
        kind: TokenizerErrorKind::ExpectedKeyword("void")
    }), "foo");

    lextest!(ok_kw_while, kw_while, "while", Ok(Token { lexeme: "while", kind: TokenKind::While }), "");
    lextest!(extra_kw_while, kw_while, "while foo", Ok(Token { lexeme: "while", kind: TokenKind::While }), " foo");
    lextest!(err_kw_while, kw_while, "foo", Err(TokenizerError {
        found: "foo",
        kind: TokenizerErrorKind::ExpectedKeyword("while")
    }), "foo");

    lextest!(ok_kw_if, kw_if, "if", Ok(Token { lexeme: "if", kind: TokenKind::If }), "");
    lextest!(extra_kw_if, kw_if, "if foo", Ok(Token { lexeme: "if", kind: TokenKind::If }), " foo");
    lextest!(err_kw_if, kw_if, "foo", Err(TokenizerError {
        found: "foo",
        kind: TokenizerErrorKind::ExpectedKeyword("if")
    }), "foo");

    lextest!(ok_kw_else, kw_else, "else", Ok(Token { lexeme: "else", kind: TokenKind::Else }), "");
    lextest!(extra_kw_else, kw_else, "else foo", Ok(Token { lexeme: "else", kind: TokenKind::Else }), " foo");
    lextest!(err_kw_else, kw_else, "foo", Err(TokenizerError {
        found: "foo",
        kind: TokenizerErrorKind::ExpectedKeyword("else")
    }), "foo");

    lextest!(ok_kw_return, kw_return, "return", Ok(Token { lexeme: "return", kind: TokenKind::Return }), "");
    lextest!(extra_kw_return, kw_return, "return foo", Ok(Token { lexeme: "return", kind: TokenKind::Return }), " foo");
    lextest!(err_kw_return, kw_return, "foo", Err(TokenizerError {
        found: "foo",
        kind: TokenizerErrorKind::ExpectedKeyword("return")
    }), "foo");

    lextest!(ok_integer, integer, "123", Ok(Token { lexeme: "123", kind: TokenKind::Int }), "");
    lextest!(extra_integer, integer, "123 foo", Ok(Token { lexeme: "123", kind: TokenKind::Int }), " foo");
    lextest!(err_integer, integer, "foo", Err(TokenizerError {
        found: "foo",
        kind: TokenizerErrorKind::ExpectedRegex("^[0-9]+")
    }), "foo");

    lextest!(ok_identifier, identifier, "foo", Ok(Token { lexeme: "foo", kind: TokenKind::Identifier }), "");
    lextest!(extra_identifier, identifier, "foo bar", Ok(Token { lexeme: "foo", kind: TokenKind::Identifier }), " bar");
    lextest!(err_identifier, identifier, "123", Err(TokenizerError {
        found: "123",
        kind: TokenizerErrorKind::ExpectedIdentifier
    }), "123");

    lextest!(ok_lparen, lparen, "(", Ok(Token { lexeme: "(", kind: TokenKind::LParen }), "");
    lextest!(extra_lparen, lparen, "( foo", Ok(Token { lexeme: "(", kind: TokenKind::LParen }), " foo");
    lextest!(err_lparen, lparen, "foo", Err(TokenizerError {
        found: "foo",
        kind: TokenizerErrorKind::ExpectedLiteral("(")
    }), "foo");

    lextest!(ok_rparen, rparen, ")", Ok(Token { lexeme: ")", kind: TokenKind::RParen }), "");
    lextest!(extra_rparen, rparen, ") foo", Ok(Token { lexeme: ")", kind: TokenKind::RParen }), " foo");
    lextest!(err_rparen, rparen, "foo", Err(TokenizerError {
        found: "foo",
        kind: TokenizerErrorKind::ExpectedLiteral(")")
    }), "foo");

    lextest!(ok_lbrace, lbrace, "{", Ok(Token { lexeme: "{", kind: TokenKind::LBrace }), "");
    lextest!(extra_lbrace, lbrace, "{ foo", Ok(Token { lexeme: "{", kind: TokenKind::LBrace }), " foo");
    lextest!(err_lbrace, lbrace, "foo", Err(TokenizerError {
        found: "foo",
        kind: TokenizerErrorKind::ExpectedLiteral("{")
    }), "foo");

    lextest!(ok_rbrace, rbrace, "}", Ok(Token { lexeme: "}", kind: TokenKind::RBrace }), "");
    lextest!(extra_rbrace, rbrace, "} foo", Ok(Token { lexeme: "}", kind: TokenKind::RBrace }), " foo");
    lextest!(err_rbrace, rbrace, "foo", Err(TokenizerError {
        found: "foo",
        kind: TokenizerErrorKind::ExpectedLiteral("}")
    }), "foo");

    lextest!(ok_lbracket, lbracket, "[", Ok(Token { lexeme: "[", kind: TokenKind::LBracket }), "");
    lextest!(extra_lbracket, lbracket, "[ foo", Ok(Token { lexeme: "[", kind: TokenKind::LBracket }), " foo");
    lextest!(err_lbracket, lbracket, "foo", Err(TokenizerError {
        found: "foo",
        kind: TokenizerErrorKind::ExpectedLiteral("[")
    }), "foo");

    lextest!(ok_rbracket, rbracket, "]", Ok(Token { lexeme: "]", kind: TokenKind::RBracket }), "");
    lextest!(extra_rbracket, rbracket, "] foo", Ok(Token { lexeme: "]", kind: TokenKind::RBracket }), " foo");
    lextest!(err_rbracket, rbracket, "foo", Err(TokenizerError {
        found: "foo",
        kind: TokenizerErrorKind::ExpectedLiteral("]")
    }), "foo");

    lextest!(ok_plus, plus, "+", Ok(Token { lexeme: "+", kind: TokenKind::Plus }), "");
    lextest!(extra_plus, plus, "+ foo", Ok(Token { lexeme: "+", kind: TokenKind::Plus }), " foo");
    lextest!(err_plus, plus, "foo", Err(TokenizerError {
        found: "foo",
        kind: TokenizerErrorKind::ExpectedLiteral("+")
    }), "foo");

    lextest!(ok_minus, minus, "-", Ok(Token { lexeme: "-", kind: TokenKind::Minus }), "");
    lextest!(extra_minus, minus, "- foo", Ok(Token { lexeme: "-", kind: TokenKind::Minus }), " foo");
    lextest!(err_minus, minus, "foo", Err(TokenizerError {
        found: "foo",
        kind: TokenizerErrorKind::ExpectedLiteral("-")
    }), "foo");

    lextest!(ok_star, star, "*", Ok(Token { lexeme: "*", kind: TokenKind::Star }), "");
    lextest!(extra_star, star, "* foo", Ok(Token { lexeme: "*", kind: TokenKind::Star }), " foo");
    lextest!(err_star, star, "foo", Err(TokenizerError {
        found: "foo",
        kind: TokenizerErrorKind::ExpectedLiteral("*")
    }), "foo");

    lextest!(ok_semicolon, semicolon, ";", Ok(Token { lexeme: ";", kind: TokenKind::Semicolon }), "");
    lextest!(extra_semicolon, semicolon, "; foo", Ok(Token { lexeme: ";", kind: TokenKind::Semicolon }), " foo");
    lextest!(err_semicolon, semicolon, "foo", Err(TokenizerError {
        found: "foo",
        kind: TokenizerErrorKind::ExpectedLiteral(";")
    }), "foo");

    lextest!(ok_colon, colon, ":", Ok(Token { lexeme: ":", kind: TokenKind::Colon }), "");
    lextest!(extra_colon, colon, ": foo", Ok(Token { lexeme: ":", kind: TokenKind::Colon }), " foo");
    lextest!(err_colon, colon, "foo", Err(TokenizerError {
        found: "foo",
        kind: TokenizerErrorKind::ExpectedLiteral(":")
    }), "foo");

    lextest!(ok_comma, comma, ",", Ok(Token { lexeme: ",", kind: TokenKind::Comma }), "");
    lextest!(extra_comma, comma, ", foo", Ok(Token { lexeme: ",", kind: TokenKind::Comma }), " foo");
    lextest!(err_comma, comma, "foo", Err(TokenizerError {
        found: "foo",
        kind: TokenizerErrorKind::ExpectedLiteral(",")
    }), "foo");

    lextest!(ok_eq, eq, "=", Ok(Token { lexeme: "=", kind: TokenKind::Eq }), "");
    lextest!(extra_eq, eq, "= foo", Ok(Token { lexeme: "=", kind: TokenKind::Eq }), " foo");
    lextest!(err_eq, eq, "foo", Err(TokenizerError {
        found: "foo",
        kind: TokenizerErrorKind::ExpectedLiteral("=")
    }), "foo");

    lextest!(ok_bang, bang, "!", Ok(Token { lexeme: "!", kind: TokenKind::Bang }), "");
    lextest!(extra_bang, bang, "! foo", Ok(Token { lexeme: "!", kind: TokenKind::Bang }), " foo");
    lextest!(err_bang, bang, "foo", Err(TokenizerError {
        found: "foo",
        kind: TokenizerErrorKind::ExpectedLiteral("!")
    }), "foo");

    lextest!(ok_qmark, qmark, "?", Ok(Token { lexeme: "?", kind: TokenKind::QMark }), "");
    lextest!(extra_qmark, qmark, "? foo", Ok(Token { lexeme: "?", kind: TokenKind::QMark }), " foo");
    lextest!(err_qmark, qmark, "foo", Err(TokenizerError {
        found: "foo",
        kind: TokenizerErrorKind::ExpectedLiteral("?")
    }), "foo");

    lextest!(ok_bin_op_plus, bin_op, "+", Ok(Token { lexeme: "+", kind: TokenKind::Plus }), "");
    lextest!(ok_bin_op_minus, bin_op, "-", Ok(Token { lexeme: "-", kind: TokenKind::Minus }), "");
    lextest!(ok_bin_op_star, bin_op, "*", Ok(Token { lexeme: "*", kind: TokenKind::Star }), "");
    lextest!(extra_bin_op, bin_op, "+ foo", Ok(Token { lexeme: "+", kind: TokenKind::Plus }), " foo");
    lextest!(err_bin_op, bin_op, "foo", Err(TokenizerError {
        found: "foo",
        kind: TokenizerErrorKind::ExpectedLiteral("*") // TODO: Handle all errors
    }), "foo");

    lextest!(ok_unary_op_minus, unary_op, "-", Ok(Token { lexeme: "-", kind: TokenKind::Minus }), "");
    lextest!(ok_unary_op_bang, unary_op, "!", Ok(Token { lexeme: "!", kind: TokenKind::Bang }), "");
    lextest!(extra_unary_op, unary_op, "- foo", Ok(Token { lexeme: "-", kind: TokenKind::Minus }), " foo");
    lextest!(err_unary_op, unary_op, "foo", Err(TokenizerError {
        found: "foo",
        kind: TokenizerErrorKind::ExpectedLiteral("!") // TODO: Handle all errors
    }), "foo");
}
