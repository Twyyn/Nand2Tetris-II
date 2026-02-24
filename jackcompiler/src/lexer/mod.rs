pub mod token;

use crate::{
    error::{LexError, LexErrorType},
    lexer::token::{Keyword, Symbol, Token, TokenType},
};
use std::str::FromStr;

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    current: usize,
    start: usize,
    line: usize,
}

impl Lexer {
    #[must_use]
    pub fn new(source: &str) -> Self {
        let source = source.chars().collect();
        Self {
            source,
            tokens: Vec::new(),
            current: 0,
            start: 0,
            line: 1,
        }
    }

    /// Scans the source and produces a slice of tokens.
    ///
    /// # Errors
    ///
    /// Returns `Err(LexError)` if a lexical error is encountered while scanning.
    pub fn scan(&mut self) -> Result<&[Token], LexError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        Ok(&self.tokens)
    }

    fn scan_token(&mut self) -> Result<(), LexError> {
        let c = self.advance();

        match c {
            // Whitespace
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,

            // Comments
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_char('*') {
                    while !self.is_at_end() {
                        if self.peek() == '\n' {
                            self.line += 1;
                        }
                        if self.peek() == '*' && self.peek_next() == '/' {
                            self.advance();
                            self.advance();
                            break;
                        }
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Symbol(Symbol::Slash));
                }
            }

            // String constant
            '"' => self.string()?,

            // Integer constant
            '0'..='9' => self.integer()?,

            // Identifiers and keywords
            'a'..='z' | 'A'..='Z' | '_' => self.identifier_or_keyword(),

            // Symbols
            _ => {
                let symbol = c.to_string();
                match Symbol::from_str(&symbol) {
                    Ok(symbol) => self.add_token(TokenType::Symbol(symbol)),
                    Err(()) => {
                        return Err(LexError {
                            kind: LexErrorType::UnexpectedChar(c),
                            line: self.line,
                        });
                    }
                }
            }
        }

        Ok(())
    }

    fn add_token(&mut self, token_type: TokenType) {
        let token = Token {
            token_type,
            lexeme: self.lexeme(),
            line: self.line,
        };
        self.tokens.push(token);
    }

    fn identifier_or_keyword(&mut self) {
        while matches!(self.peek(), 'a'..='z' | 'A'..='Z' | '0'..='9' | '_') {
            self.advance();
        }

        let lexeme = self.lexeme();
        let token_type = match Keyword::from_str(&lexeme) {
            Ok(keyword) => TokenType::Keyword(keyword),
            Err(()) => TokenType::Identifier(lexeme.clone()),
        };

        self.tokens.push(Token {
            lexeme,
            token_type,
            line: self.line,
        });
    }

    fn string(&mut self) -> Result<(), LexError> {
        let line = self.line;

        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                return Err(LexError {
                    kind: LexErrorType::UnterminatedString,
                    line,
                });
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(LexError {
                kind: LexErrorType::UnterminatedString,
                line,
            });
        }

        self.advance();

        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token(TokenType::StringConstant(value));
        Ok(())
    }

    fn integer(&mut self) -> Result<(), LexError> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        let lexeme = self.lexeme();
        match lexeme.parse::<u16>() {
            Ok(n) if n <= 32767 => self.add_token(TokenType::IntegerConstant(n)),
            _ => {
                return Err(LexError {
                    kind: LexErrorType::InvalidIntConstant(lexeme),
                    line: self.line,
                });
            }
        }

        Ok(())
    }

    fn lexeme(&self) -> String {
        self.source[self.start..self.current].iter().collect()
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.peek() != expected {
            return false;
        }
        self.advance();
        true
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(&self) -> char {
        self.source.get(self.current).copied().unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.source.get(self.current + 1).copied().unwrap_or('\0')
    }
}
