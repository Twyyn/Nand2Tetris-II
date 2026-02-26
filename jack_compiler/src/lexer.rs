use crate::token::{Keyword, Span, Symbol, Token, TokenKind};
use std::str::FromStr;

pub struct Lexer<'src> {
    source: &'src str,
    source_as_bytes: &'src [u8],
    pos: usize,
    line: u32,
    column: u16,
    tokens: Vec<Token<'src>>,
}

impl<'src> Lexer<'src> {
    #[must_use]
    pub fn new(source: &'src str) -> Self {
        let source_as_bytes = source.as_bytes();
        Self {
            source, // Raw source
            source_as_bytes,
            pos: 0,
            line: 1,
            column: 1,
            tokens: Vec::new(),
        }
    }

    #[must_use]
    pub fn tokenize(mut self) -> Vec<Token<'src>> {
        while !self.is_at_end() {
            self.scan_token();
        }
        self.add_token(TokenKind::Eof, self.pos);
        self.tokens
    }

    // Scanner Dispatch
    #[rustfmt::skip]
    fn scan_token(&mut self) {
        let start = self.pos;
        let c = self.advance();

        match c {
            b'/' if self.peek() == b'*' || self.peek() == b'/' => {
                self.skip_comment();
            }
            _ if c.is_ascii_whitespace() => {
                self.advance_while(|b| b.is_ascii_whitespace());
            }

            b'"'                             => self.scan_string(start),
            b'0'..=b'9'                      => self.scan_integer(start),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.scan_word(start),
            _                                => self.scan_symbol(start),
        }
    }

    // --- Token Helper ---
    #[allow(clippy::cast_possible_truncation)]
    fn add_token(&mut self, kind: TokenKind<'src>, start: usize) {
        let len = if matches!(kind, TokenKind::Eof) {
            0
        } else {
            self.pos - start
        };
        let span = Span::new(
            start as u32,
            len as u16,
            self.line,
            self.column.saturating_sub(len as u16),
        );
        self.tokens.push(Token::new(kind, span));
    }

    // --- Comments ---
    fn skip_comment(&mut self) {
        match self.peek() {
            // Block Comment
            b'*' => {
                self.advance(); // Skip '*'
                while !self.is_at_end() {
                    if self.peek() == b'*' && self.peek_next() == b'/' {
                        self.advance(); // Skip '*'
                        self.advance(); // Skip '/'
                        break;
                    }
                    self.advance();
                }
            }
            //Inline Comment
            _ => self.advance_while(|b| b != b'\n'),
        }
    }

    // --- Character/Byte Navigation Helpers ---
    fn is_at_end(&self) -> bool {
        self.pos >= self.source_as_bytes.len()
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            b'\0'
        } else {
            self.source_as_bytes[self.pos]
        }
    }

    fn peek_next(&self) -> u8 {
        if self.pos + 1 >= self.source_as_bytes.len() {
            b'\0'
        } else {
            self.source_as_bytes[self.pos + 1]
        }
    }

    fn advance(&mut self) -> u8 {
        let current_byte = self.source_as_bytes[self.pos];
        self.pos += 1;
        if current_byte == b'\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        current_byte
    }

    fn advance_while(&mut self, predicate: fn(u8) -> bool) {
        while !self.is_at_end() && predicate(self.peek()) {
            self.advance();
        }
    }

    fn slice(&self, start: usize, end: usize) -> &'src str {
        &self.source[start..end]
    }

    // --- Scanner Helpers ---
    fn scan_integer(&mut self, start: usize) {
        self.advance_while(|b| b.is_ascii_digit());
        let lexeme = self.slice(start, self.pos);
        let value = match lexeme.parse::<u16>() {
            Ok(value) if value <= 32767 => value,
            Ok(_) => todo!(),  // TODO: Integer out of range error
            Err(_) => todo!(), // TODO: Proper error
        };
        self.add_token(TokenKind::IntegerConstant(value), start);
    }

    fn scan_string(&mut self, start: usize) {
        let string_start = self.pos;
        self.advance_while(|b| b != b'"');
        let lexeme = self.slice(string_start, self.pos);
        if !self.is_at_end() && self.peek() == b'"' {
            self.advance();
        } else {
            todo!("Unterminated String") // TODO: else emit an "unterminated string" error
        }
        self.add_token(TokenKind::StringConstant(lexeme), start);
    }

    fn scan_word(&mut self, start: usize) {
        self.advance_while(|b| b.is_ascii_alphanumeric() || b == b'_');
        let lexeme = self.slice(start, self.pos);
        let kind = match Keyword::from_str(lexeme) {
            Ok(keyword) => TokenKind::Keyword(keyword),
            Err(()) => TokenKind::Identifier(lexeme),
        };
        self.add_token(kind, start);
    }

    fn scan_symbol(&mut self, start: usize) {
        let c = self.source_as_bytes[self.pos - 1] as char;
        let kind = match Symbol::from_char(c) {
            Some(symbol) => TokenKind::Symbol(symbol),
            None => todo!(), //TODO Add Proper Error Handling
        };
        self.add_token(kind, start);
    }
}
