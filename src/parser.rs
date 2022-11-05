use std::iter::Peekable;

use crate::lexer::{Lexer, Token, TokenKind};

#[derive(Debug)]
pub enum ParseError {
    SyntaxError(Option<usize>, String),
}

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
        }
    }

    pub fn parse(self) -> Result<Vec<String>, ParseError> {
        let tokens: Vec<Token> = self.lexer.collect();

        let mut segments: Vec<String> = Vec::new();
        let mut current_segment = String::new();

        for token in tokens {
            match token.kind {
                TokenKind::Alpha | TokenKind::Number | TokenKind::Dash => {
                    current_segment += token.lexeme;
                }
                TokenKind::Delim => {
                    segments.push(current_segment);
                    current_segment = String::new();
                }
                TokenKind::Star => {
                    current_segment += token.lexeme;
                }
                TokenKind::Error => Err(ParseError::SyntaxError(None, token.lexeme.to_string()))?,
            }
        }

        if !current_segment.is_empty() {
            segments.push(current_segment);
        }

        Ok(segments)
    }
}
