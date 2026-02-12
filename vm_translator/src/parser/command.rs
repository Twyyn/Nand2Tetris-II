use crate::error::ParseError;

use std::{
    fmt::{self},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Push { segment: Seg, index: u16 },
    Pop { segment: Seg, index: u16 },
    Operation(Op),
    Branch(Br),
    Function(Func),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Seg {
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
pub enum Op {
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
pub enum Br {
    Label { label: String },
    Goto { label: String },
    IfGoto { label: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Func {
    Define { name: String, n_vars: u16 },
    Call { function: String, n_args: u16 },
    Return,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Push { segment, index } => write!(f, "push {segment} {index}"),
            Self::Pop { segment, index } => write!(f, "pop {segment} {index}"),
            Self::Operation(operation) => write!(f, "{operation}"),
            Self::Branch(branch) => write!(f, "{branch}"),
            Self::Function(function) => write!(f, "{function}"),
        }
    }
}

impl fmt::Display for Seg {
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

impl fmt::Display for Op {
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

impl fmt::Display for Br {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Label { label } => write!(f, "label {label}"),
            Self::Goto { label } => write!(f, "goto {label}"),
            Self::IfGoto { label } => write!(f, "if-goto {label}"),
        }
    }
}

impl fmt::Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Define { name, n_vars } => write!(f, "function {name} {n_vars}"),
            Self::Call { function, n_args } => write!(f, "call {function} {n_args}"),
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

                let segment: Seg = segment
                    .parse()
                    .map_err(|()| ParseError::InvalidSegment(segment.to_string()))?;

                let index: u16 = index
                    .parse()
                    .map_err(|_| ParseError::InvalidIndex(index.to_string()))?;

                if segment == Seg::Temp && index > 7 {
                    return Err(ParseError::IndexOutOfRange {
                        segment: segment.to_string(),
                        index,
                        max: 7,
                    });
                }

                if segment == Seg::Pointer && index > 1 {
                    return Err(ParseError::IndexOutOfRange {
                        segment: segment.to_string(),
                        index,
                        max: 1,
                    });
                }

                if command == "pop" {
                    if segment == Seg::Constant {
                        return Err(ParseError::CannotPopConstant);
                    }
                    Ok(Command::Pop { segment, index })
                } else {
                    Ok(Command::Push { segment, index })
                }
            }
            // Branching Commands
            (Some("label"), Some(label), None) => Ok(Command::Branch(Br::Label {
                label: label.to_string(),
            })),
            (Some("goto"), Some(label), None) => Ok(Command::Branch(Br::Goto {
                label: label.to_string(),
            })),

            (Some("if-goto"), Some(label), None) => Ok(Command::Branch(Br::IfGoto {
                label: label.to_string(),
            })),
            (Some("label" | "goto" | "if-goto"), None, None) => {
                Err(ParseError::MissingLabel(s.to_string()))
            }
            // Function Commands
            (Some("function"), Some(name), Some(n_vars)) => {
                if tokens.next().is_some() {
                    return Err(ParseError::UnknownCommand(s.to_string()));
                }

                let n_vars: u16 = n_vars
                    .parse()
                    .map_err(|_| ParseError::MissingVarCount(n_vars.to_string()))?;

                Ok(Command::Function(Func::Define {
                    name: name.to_string(),
                    n_vars,
                }))
            }
            (Some("call"), Some(function), Some(n_args)) => {
                if tokens.next().is_some() {
                    return Err(ParseError::UnknownCommand(s.to_string()));
                }

                let n_args: u16 = n_args
                    .parse()
                    .map_err(|_| ParseError::MissingArgCount(n_args.to_string()))?;

                Ok(Command::Function(Func::Call {
                    function: function.to_string(),
                    n_args,
                }))
            }
            (Some("return"), None, None) => Ok(Command::Function(Func::Return)),
            // Arithmetic & Logical Commands
            (Some(command), None, None) => command
                .parse::<Op>()
                .map(Command::Operation)
                .map_err(|()| ParseError::UnknownCommand(command.to_string())),

            _ => Err(ParseError::UnknownCommand(s.to_string())),
        }
    }
}

impl FromStr for Seg {
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

impl FromStr for Op {
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
