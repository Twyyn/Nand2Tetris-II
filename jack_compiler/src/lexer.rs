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
            // Raw source
            source,
            source_as_bytes,
            pos: 0,
            line: 1,
            column: 1,
            tokens: Vec::new(),
        }
    }

    // --- Scanners ---

    fn scan_integer(&mut self, start: usize) {
        self.advance_while(|b| b.is_ascii_digit());
        let lexme = self.slice(start, self.pos);
        let value: u16 = lexme.parse().unwrap_or(0);
        self.add_token(TokenKind::IntegerConstant(value), start);
    }

    fn scan_string(&mut self) {
        let start = self.pos;
        self.advance_while(|b| b != b'"');
        let lexeme = self.slice(start, self.pos).to_string();
        self.add_token(TokenKind::StringConstant(lexeme), start);
    }

    fn scan_symbol(&mut self) {
        
    }

    // --- Whitespace & Comments ---

    fn skip_whitespace(&mut self) {
        self.advance_while(|b| b.is_ascii_whitespace());
    }

    fn skip_line_comment(&mut self) {
        self.advance_while(|b| b != b'\n');
    }

    fn skip_block_comment(&mut self) {
        self.advance(); // skip '/'
        self.advance(); // skip '*'
        while !self.is_at_end() {
            if self.peek() == b'*' && self.peek_next() == b'/' {
                self.advance(); // skip '*'
                self.advance(); // skip '/'
                break;
            }
            self.advance();
        }
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

    // --- Token Helpers ---
    #[allow(clippy::cast_possible_truncation)]
    fn add_token(&mut self, kind: TokenKind, start: usize) {
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
}
