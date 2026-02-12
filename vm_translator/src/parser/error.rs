use std::fmt;

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
    MissingLabel(String),
    MissingVarCount(String),
    MissingArgCount(String),
}

impl std::error::Error for ParseError {}

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
            } => write!(f, "Invalid index {index} for {segment} (expected 0–{max})"),

            Self::MissingLabel(_) => todo!(),
            Self::MissingVarCount(_) => todo!(),
            Self::MissingArgCount(_) => todo!(),
        }
    }
}
