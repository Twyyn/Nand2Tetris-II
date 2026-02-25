mod keyword;
mod kind;
mod symbol;

pub use keyword::Keyword;
pub use kind::TokenKind;
pub use symbol::Symbol;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub offset: u32,
    pub len: u16,
    pub line: u32,
    pub column: u16,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn lexeme<'a>(&self, source: &'a str) -> &'a str {
        let start = self.span.offset as usize;
        &source[start..start + self.span.len as usize]
    }
}

impl Span {
    pub fn new(offset: u32, len: u16, line: u32, column: u16) -> Self {
        Self {
            offset,
            len,
            line,
            column,
        }
    }
}
