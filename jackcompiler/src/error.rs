use std::fmt;

#[derive(Debug)]
pub enum CompilerError {
    LexError(LexError),
}

#[derive(Debug)]
pub enum LexErrorKind {
    UnexpectedChar(char),
    UnterminatedString,
    InvalidIntConstant(String),
    InvalidStringConstant(String),
}

#[derive(Debug)]
pub struct LexError {
    pub kind: LexErrorKind,
    pub line: usize,
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::LexError(e) => write!(f, "{e}"),
        }
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.line, self.kind)
    }
}

impl fmt::Display for LexErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexErrorKind::UnexpectedChar(c) => write!(f, "unexpected character '{c}'"),
            LexErrorKind::UnterminatedString => write!(f, "unterminated string literal"),
            LexErrorKind::InvalidIntConstant(s) => write!(f, "invalid integer constant '{s}'"),
            LexErrorKind::InvalidStringConstant(s) => write!(f, "invalid string constant '{s}'"),
        }
    }
}
