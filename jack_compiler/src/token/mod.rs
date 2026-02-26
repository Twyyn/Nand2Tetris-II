mod error;
mod keyword;
mod kind;
mod symbol;

pub use error::TokenError;
pub use keyword::Keyword;
pub use kind::TokenKind;
pub use symbol::Symbol;

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub offset: u32,
    pub len: u16,
    pub line: u32,
    pub column: u16,
}

impl<'a> Token<'a> {
    #[must_use]
    pub fn new(kind: TokenKind<'a>, span: Span) -> Self {
        Self { kind, span }
    }

    #[must_use]
    pub fn lexeme(&self, source: &'a str) -> &'a str {
        let start = self.span.offset as usize;
        &source[start..start + self.span.len as usize]
    }

    #[must_use]
    pub fn as_xml(&self, source: &str) -> String {
        let kind = &self.kind;
        let lexeme = self.lexeme(source);
        if lexeme.is_empty() {
            format!("<{kind}></{kind}>")
        } else {
            format!("<{kind}> {lexeme} </{kind}>")
        }
    }
}

impl Span {
    #[must_use]
    pub fn new(offset: u32, len: u16, line: u32, column: u16) -> Self {
        Self {
            offset,
            len,
            line,
            column,
        }
    }
}
