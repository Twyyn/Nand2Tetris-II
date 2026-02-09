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
}
