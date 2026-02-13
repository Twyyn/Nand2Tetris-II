use crate::error::ParseError;

use std::{
    fmt::{self},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Push { segment: Segment, index: u16 },
    Pop { segment: Segment, index: u16 },
    Operation(OperationCommand),
    Branch(BranchCommand),
    Function(FunctionCommand),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationCommand {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BranchCommand {
    Label { label: String },
    Goto { label: String },
    IfGoto { label: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionCommand {
    Function { name: String, local_count: u16 },
    Call { name: String, arg_count: u16 },
    Return,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Push { segment, index } => write!(f, "push {segment} {index}"),
            Self::Pop { segment, index } => write!(f, "pop {segment} {index}"),
            Self::Operation(operation_command) => write!(f, "{operation_command}"),
            Self::Branch(branch_command) => write!(f, "{branch_command}"),
            Self::Function(function_command) => write!(f, "{function_command}"),
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

impl fmt::Display for OperationCommand {
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

impl fmt::Display for BranchCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Label { label } => write!(f, "label {label}"),
            Self::Goto { label } => write!(f, "goto {label}"),
            Self::IfGoto { label } => write!(f, "if-goto {label}"),
        }
    }
}

impl fmt::Display for FunctionCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Function { name, local_count } => write!(f, "function {name} {local_count}"),
            Self::Call { name, arg_count } => write!(f, "call {name} {arg_count}"),
            Self::Return => write!(f, "return"),
        }
    }
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();

        match (tokens.next(), tokens.next(), tokens.next()) {
            // Stack Commands
            (Some(command @ ("push" | "pop")), Some(segment), Some(index)) => {
                if tokens.next().is_some() {
                    return Err(ParseError::UnknownCommand(s.to_string()));
                }

                let segment: Segment = segment
                    .parse()
                    .map_err(|()| ParseError::InvalidSegment(segment.to_string()))?;

                let index: u16 = index
                    .parse()
                    .map_err(|_| ParseError::InvalidIndex(index.to_string()))?;

                match segment {
                    Segment::Temp if index > 7 => {
                        return Err(ParseError::IndexOutOfRange {
                            segment: segment.to_string(),
                            index,
                            max: 7,
                        });
                    }
                    Segment::Pointer if index > 1 => {
                        return Err(ParseError::IndexOutOfRange {
                            segment: segment.to_string(),
                            index,
                            max: 1,
                        });
                    }
                    _ => {}
                }

                if command == "pop" && segment == Segment::Constant {
                    return Err(ParseError::CannotPopConstant);
                }

                match command {
                    "push" => Ok(Command::Push { segment, index }),
                    "pop" => Ok(Command::Pop { segment, index }),

                    _ => unreachable!(),
                }
            }
            // Branching Commands
            (Some(command @ ("label" | "goto" | "if-goto")), Some(label), None) => {
                let label = label.to_string();

                Ok(Command::Branch(match command {
                    "label" => BranchCommand::Label { label },
                    "goto" => BranchCommand::Goto { label },
                    "if-goto" => BranchCommand::IfGoto { label },

                    _ => unreachable!(),
                }))
            }
            // Function Commands
            (Some(command @ ("function" | "call")), Some(name), Some(n)) => {
                if tokens.next().is_some() {
                    return Err(ParseError::UnknownCommand(s.to_string()));
                }

                let name = name.to_string();
                let n: u16 = n.parse().map_err(|_| match command {
                    "function" => ParseError::InvalidVarCount(n.to_string()),
                    _ => ParseError::InvalidAarCount(n.to_string()),
                })?;

                Ok(Command::Function(match command {
                    "function" => FunctionCommand::Function {
                        name,
                        local_count: n,
                    },
                    "call" => FunctionCommand::Call { name, arg_count: n },

                    _ => unreachable!(),
                }))
            }
            (Some("return"), None, None) => Ok(Command::Function(FunctionCommand::Return)),
            // Arithmetic & Logical Commands
            (Some(command), None, None) => command
                .parse::<OperationCommand>()
                .map(Command::Operation)
                .map_err(|()| ParseError::UnknownCommand(command.to_string())),

            _ => Err(ParseError::UnknownCommand(s.to_string())),
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

impl FromStr for OperationCommand {
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
