use super::errors::ParseError;
use std::{
    fmt::{self},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Push { segment: Segment, index: u16 },
    Pop { segment: Segment, index: u16 },
    Arithmetic(ArithmeticCommand),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Segment {
    Constant,
    Local,
    Argument,
    This,
    That,
    Static,
    Temp,
    Pointer,
}

impl fmt::Display for ArithmeticCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => write!(f, "add"),
            Self::Sub => write!(f, "sub"),
            Self::Neg => write!(f, "neg"),
            Self::Eq => write!(f, "eq"),
            Self::Gt => write!(f, "gt"),
            Self::Lt => write!(f, "lt"),
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
            Self::Not => write!(f, "not"),
        }
    }
}

impl FromStr for ArithmeticCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" => Ok(Self::Add),
            "sub" => Ok(Self::Sub),
            "neg" => Ok(Self::Neg),
            "eq" => Ok(Self::Eq),
            "gt" => Ok(Self::Gt),
            "lt" => Ok(Self::Lt),
            "and" => Ok(Self::And),
            "or" => Ok(Self::Or),
            "not" => Ok(Self::Not),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Constant => write!(f, "constant"),
            Self::Local => write!(f, "local"),
            Self::Argument => write!(f, "argument"),
            Self::This => write!(f, "this"),
            Self::That => write!(f, "that"),
            Self::Static => write!(f, "static"),
            Self::Temp => write!(f, "temp"),
            Self::Pointer => write!(f, "pointer"),
        }
    }
}

impl FromStr for Segment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "constant" => Ok(Self::Constant),
            "local" => Ok(Self::Local),
            "argument" => Ok(Self::Argument),
            "this" => Ok(Self::This),
            "that" => Ok(Self::That),
            "static" => Ok(Self::Static),
            "temp" => Ok(Self::Temp),
            "pointer" => Ok(Self::Pointer),
            _ => Err(()),
        }
    }
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
            Self::IndexOutOfRange {
                segment,
                index,
                max,
            } => write!(f, "Invalid index {index} for {segment} (expected 0–{max})"),
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

                match (command, segment) {
                    ("pop", Segment::Constant) => {
                        return Err(ParseError::CannotPopConstant);
                    }
                    (_, Segment::Pointer) if index > 1 => {
                        return Err(ParseError::IndexOutOfRange {
                            segment: segment.to_string(),
                            index,
                            max: 1,
                        });
                    }
                    (_, Segment::Temp) if index > 7 => {
                        return Err(ParseError::IndexOutOfRange {
                            segment: segment.to_string(),
                            index,
                            max: 7,
                        });
                    }
                    _ => {}
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
