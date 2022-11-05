use std::iter::Peekable;

use crate::lexer::{Lexer, Token, TokenKind};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseNsidError {
    TooFewSegments,
    SyntaxError(String),
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

    pub fn parse(self) -> Result<Vec<String>, ParseNsidError> {
        let tokens: Vec<Token> = self.lexer.collect();

        let mut segments: Vec<String> = Vec::new();
        let mut current_segment: Option<String> = None;

        let tokens_len = tokens.len();
        for (index, token) in tokens.into_iter().enumerate() {
            match current_segment.as_mut() {
                Some(segment) => match token.kind {
                    TokenKind::Alpha | TokenKind::Number | TokenKind::Dash => {
                        *segment += token.lexeme;
                    }
                    TokenKind::Delim => {
                        segments.push(segment.to_string());
                        current_segment = None;
                    }
                    TokenKind::Star => {
                        *segment += token.lexeme;
                    }
                    TokenKind::Error => Err(ParseNsidError::SyntaxError(token.lexeme.to_string()))?,
                },
                None => match token.kind {
                    TokenKind::Alpha => {
                        current_segment = Some(token.lexeme.to_string());
                    }
                    TokenKind::Number | TokenKind::Dash | TokenKind::Delim => {
                        Err(ParseNsidError::SyntaxError(token.lexeme.to_string()))?
                    }
                    TokenKind::Star => {
                        if index == tokens_len - 1 {
                            current_segment = Some(token.lexeme.to_string())
                        } else {
                            Err(ParseNsidError::SyntaxError(token.lexeme.to_string()))?
                        }
                    }
                    TokenKind::Error => Err(ParseNsidError::SyntaxError(token.lexeme.to_string()))?,
                },
            }
        }

        if let Some(current_segment) = current_segment {
            segments.push(current_segment);
        }

        Ok(segments)
    }
}
