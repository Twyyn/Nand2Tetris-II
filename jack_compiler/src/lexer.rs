use crate::token::{Keyword, Span, Symbol, Token, TokenKind};

pub struct Lexer<'src> {
    source: &'src str,
    source_as_bytes: &'src [u8],
    pos: usize,
    line: u32,
    column: u16,
    tokens: Vec<Token>,
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src str) -> Self {
        let source_as_bytes = source.as_bytes();
        Self {
            source,
            source_as_bytes,
            pos: 0,
            line: 1,
            column: 1,
            tokens: Vec::new(),
        }
    }

    fn slice(&self, start: usize, end: usize) -> &'src str {
        &self.source[start..end]
    }

    // --- Character Helpers ---
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
}
