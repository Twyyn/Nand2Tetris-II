use std::{error, fmt, io};

#[derive(Debug)]
pub enum CompilerError {
    InvalidInput(String),
    LexError(LexError),
    IO(std::io::Error),
}

#[derive(Debug)]
pub enum LexErrorType {
    UnexpectedChar(char),
    UnterminatedString,
    InvalidIntConstant(String),
    InvalidStringConstant(String),
}

#[derive(Debug)]
pub struct ParseError {
    pub line: usize,
    pub location: String,
    pub message: String,
}

#[derive(Debug)]
pub struct LexError {
    pub kind: LexErrorType,
    pub line: usize,
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LexError(e) => write!(f, "{e}"),
            Self::IO(e) => write!(f, "IO error: {e}"),
            Self::InvalidInput(msg) => write!(f, "{msg}"),
        }
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.line, self.kind)
    }
}

impl fmt::Display for LexErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedChar(c) => write!(f, "unexpected character '{c}'"),
            Self::UnterminatedString => write!(f, "unterminated string literal"),
            Self::InvalidIntConstant(s) => write!(f, "invalid integer constant '{s}'"),
            Self::InvalidStringConstant(s) => write!(f, "invalid string constant '{s}'"),
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[line {}] Error {}: {}",
            self.line, self.location, self.message
        )
    }
}

impl error::Error for LexError {}

impl error::Error for ParseError {}

impl error::Error for CompilerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidInput(_) => None,
            Self::LexError(e) => Some(e),
            Self::IO(e) => Some(e),
        }
    }
}

impl From<LexError> for CompilerError {
    fn from(e: LexError) -> Self {
        Self::LexError(e)
    }
}

impl From<io::Error> for CompilerError {
    fn from(e: io::Error) -> Self {
        Self::IO(e)
    }
}
