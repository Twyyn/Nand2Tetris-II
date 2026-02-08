#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidSegment(String),
    InvalidIndex(String),
    CannotPopConstant,
    UnknownCommand(String),
}
