use std::{error, fmt, io};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    CannotPopConstant,
    InvalidSegment(String),
    InvalidIndex(String),
    InvalidVarCount(String),
    InvalidArgCount(String),
    InvalidLabel(String),
    UnknownCommand(String),
    InvalidConstant {
        value: u16,
    },
    IndexOutOfRange {
        segment: String,
        index: u16,
        max: u16,
    },
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
            Self::CannotPopConstant => write!(f, "Cannot pop to constant segment"),
            Self::InvalidSegment(s) => write!(f, "Invalid segment: {s}"),
            Self::InvalidIndex(s) => write!(f, "Invalid index: {s}"),
            Self::InvalidLabel(s) => write!(f, "Invalid label {s}"),
            Self::InvalidVarCount(s) => write!(f, "Invalid variable count: {s}"),
            Self::InvalidArgCount(s) => write!(f, "Invalid argument count: {s}"),
            Self::UnknownCommand(s) => {
                write!(f, "Unknown command: {s}",)
            }
            Self::InvalidConstant { value } => {
                write!(f, "Constant {value} exceeds 15-bit max (32767)")
            }
            Self::IndexOutOfRange {
                segment,
                index,
                max,
            } => {
                write!(f, "Invalid index {index} for {segment} (expected 0–{max})")
            }
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
