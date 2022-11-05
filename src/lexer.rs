use std::ops::Range;

use logos::Logos;

#[derive(Debug)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub lexeme: &'a str,
    pub span: Range<usize>,
}

// Grammar:
//
// alpha     = "a" / "b" / "c" / "d" / "e" / "f" / "g" / "h" / "i" / "j" / "k" / "l" / "m" / "n" / "o" / "p" / "q" / "r" / "s" / "t" / "u" / "v" / "w" / "x" / "y" / "z" / "A" / "B" / "C" / "D" / "E" / "F" / "G" / "H" / "I" / "J" / "K" / "L" / "M" / "N" / "O" / "P" / "Q" / "R" / "S" / "T" / "U" / "V" / "W" / "X" / "Y" / "Z"
// number    = "1" / "2" / "3" / "4" / "5" / "6" / "7" / "8" / "9" / "0"
// delim     = "."
// segment   = alpha *( alpha / number / "-" )
// authority = segment *( delim segment )
// name      = segment
// nsid      = authority delim name
// nsid-ns   = authority delim "*"

#[derive(Logos, Debug, PartialEq)]
pub enum TokenKind {
    #[regex("[a-zA-Z]+")]
    Alpha,

    #[regex("[0-9]+")]
    Number,

    #[token("-")]
    Dash,

    #[token(".")]
    Delim,

    #[token("*")]
    Star,

    #[error]
    Error,
}

pub struct Lexer<'a> {
    lexer: logos::Lexer<'a, TokenKind>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: TokenKind::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.lexer.next()?;
        let lexeme = self.lexer.slice();
        let span = self.lexer.span();

        Some(Token {
            kind: kind.into(),
            lexeme,
            span,
        })
    }
}
