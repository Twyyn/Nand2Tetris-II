use super::{Keyword, Symbol};

pub type Identifier = String;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Keyword(Keyword),
    Symbol(Symbol),
    IntegerConstant(u16),
    StringConstant(String),
    Identifier(Identifier),
    Eof,
}

impl TokenKind {
    pub fn is_keyword(&self) -> bool {
        matches!(self, Self::Keyword(_))
    }

    pub fn is_symbol(&self) -> bool {
        matches!(self, Self::Symbol(_))
    }

    pub fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier(_))
    }
}
