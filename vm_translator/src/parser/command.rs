use super::error::ParseError;

use std::{
    fmt::{self},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Push { segment: Segment, index: u16 },
    Pop { segment: Segment, index: u16 },
    Operation { operation: OP },
    Branch { branch: BR },
    Function { function: FN },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OP {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BR {
    Label { label: String },
    Jump { label: String },
    JumpIf { label: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FN {
    Define { name: String, n_vars: u16 },
    Call { function: String, n_args: u16 },
    Return,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Push { segment, index } => write!(f, "push {segment} {index}"),
            Self::Pop { segment, index } => write!(f, "pop {segment} {index}"),
            Self::Operation { operation } => write!(f, "{operation}"),
            Self::Branch { branch } => write!(f, "{branch}"),
            Self::Function { function } => write!(f, "{function}"),
        }
    }
}

impl<'a> fmt::Display for Segment {
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

impl<'a> fmt::Display for OP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
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

impl fmt::Display for BR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Label { label } => write!(f, "label {label}"),
            Self::Jump { label } => write!(f, "goto {label}"),
            Self::JumpIf { label } => write!(f, "if-goto {label}"),
        }
    }
}

impl fmt::Display for FN {
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
            /* Memory Commands */
            (Some(command @ ("push" | "pop")), Some(segment), Some(index)) => {
                let segment: Segment = segment
                    .parse()
                    .map_err(|()| ParseError::InvalidSegment(segment.to_string()))?;
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
            /* Branch Commands */
            (Some("label"), Some(label), None) => Ok(Command::Branch {
                branch: BR::Label {
                    label: label.to_string(),
                },
            }),
            (Some("goto"), Some(label), None) => Ok(Command::Branch {
                branch: BR::Jump {
                    label: label.to_string(),
                },
            }),

            (Some("if-goto"), Some(label), None) => Ok(Command::Branch {
                branch: BR::JumpIf {
                    label: label.to_string(),
                },
            }),
            /* Function Commands */
            (Some("Function"), Some(name), Some(n_vars)) => {
                let n_vars: u16 = n_vars
                    .parse()
                    .map_err(|_| ParseError::MissingVarCount(n_vars.to_string()))?;

                Ok(Command::Function {
                    function: FN::Define {
                        name: name.to_string(),
                        n_vars,
                    },
                })
            }
            (Some("Call"), Some(function), Some(n_args)) => {
                let n_args: u16 = n_args
                    .parse()
                    .map_err(|_| ParseError::MissingArgCount(n_args.to_string()))?;

                Ok(Command::Function {
                    function: FN::Call {
                        function: function.to_string(),
                        n_args,
                    },
                })
            }

            (Some("return"), None, None) => Ok(Command::Function {
                function: FN::Return,
            }),
            (Some(command), None, None) => command
                .parse::<OP>()
                .map(|operation| Command::Operation { operation })
                .map_err(|_| ParseError::UnknownCommand(command.to_string())),

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

impl FromStr for OP {
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
