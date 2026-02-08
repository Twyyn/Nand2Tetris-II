use super::errors::ParseError;
use std::fmt::{self};
use std::str::FromStr;
use strum::EnumString;

#[derive(Debug, PartialEq, Eq)]

pub enum Command {
    Push { segment: Segment, index: u16 },
    Pop { segment: Segment, index: u16 },
    Arithmetic(ArithmeticCommand),
}

#[derive(Debug, Clone, Copy, EnumString, strum::Display, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum ArithmeticCommand {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, Copy, EnumString, strum::Display, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum Segment {
    Constant,
    Local,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Push { segment, index } => write!(f, "push {segment} {index}"),
            Self::Pop { segment, index } => write!(f, "pop {segment} {index}"),
            Self::Arithmetic(command) => write!(f, "{command}"),
        }
    }
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSegment(s) => write!(f, "Invalid segment: {s}"),
            Self::InvalidIndex(s) => write!(f, "Invalid index: {s}"),
            Self::CannotPopConstant => write!(f, "Cannot pop to constant segment"),
            Self::UnknownCommand(s) => write!(f, "Unknown command: {s}"),
        }
    }
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();

        match (tokens.next(), tokens.next(), tokens.next()) {
            (Some(command @ ("push" | "pop")), Some(segment), Some(index)) => {
                let segment: Segment = segment
                    .parse()
                    .map_err(|_| ParseError::InvalidSegment(segment.to_string()))?;
                let index: u16 = index
                    .parse()
                    .map_err(|_| ParseError::InvalidIndex(index.to_string()))?;

                if command == "pop" && segment == Segment::Constant {
                    return Err(ParseError::CannotPopConstant);
                }

                if command == "push" {
                    Ok(Command::Push { segment, index })
                } else {
                    Ok(Command::Pop { segment, index })
                }
            }

            (Some(command), None, None) => command
                .parse::<ArithmeticCommand>()
                .map(Command::Arithmetic)
                .map_err(|_| ParseError::UnknownCommand(command.to_string())),
            _ => Err(ParseError::UnknownCommand(s.to_string())),
        }
    }
}
