use std::{error, fmt, io};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    CannotPopConstant,
    InvalidSegment(String),
    InvalidIndex(String),
    IndexOutOfRange {
        segment: String,
        index: u16,
        max: u16,
    },
    UnknownCommand(String),
    InvalidVarCount(String),
    InvalidAarCount(String),
}

#[derive(Debug)]
pub enum VMError {
    Parse { line: usize, source: ParseError },
    IO(std::io::Error),
    InvalidInput(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSegment(s) => write!(f, "Invalid segment: {s}"),
            Self::InvalidIndex(s) => write!(f, "Invalid index: {s}"),
            Self::CannotPopConstant => write!(f, "Cannot pop to constant segment"),
            Self::UnknownCommand(s) => write!(f, "Unknown command: {s}"),
            Self::IndexOutOfRange {
                segment,
                index,
                max,
            } => {
                write!(f, "Invalid index {index} for {segment} (expected 0–{max})")
            }
            Self::InvalidVarCount(s) => write!(f, "Invalid variable count: {s}"),
            Self::InvalidAarCount(s) => write!(f, "Invalid argument count: {s}"),
        }
    }
}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse { line, source } => write!(f, "Parse error at line {line}: {source}"),
            Self::IO(e) => write!(f, "IO error: {e}"),
            Self::InvalidInput(msg) => write!(f, "{msg}"),
        }
    }
}

impl error::Error for ParseError {}

impl error::Error for VMError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Parse { source, .. } => Some(source),
            Self::IO(e) => Some(e),
            Self::InvalidInput(_) => None,
        }
    }
}

impl From<io::Error> for VMError {
    fn from(e: io::Error) -> Self {
        Self::IO(e)
    }
}
