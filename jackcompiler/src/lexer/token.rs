use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenType,
    pub lexeme: String,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Keyword(Keyword),
    Symbol(Symbol),
    IntegerConstant(u16),
    StringConstant(String),
    Identifier(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keyword {
    Class,
    Constructor,
    Function,
    Method,
    Field,
    Void,
    Static,
    Var,
    DO,
    IF,
    Else,
    While,
    Let,
    Return,
    True,
    False,
    This,
    That,
    Char,
    Int,
    Boolean,
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Symbol {
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Dot,
    Comma,
    Semicolon,
    Plus,
    Minus,
    Star,
    Slash,
    Ampersign,
    Pipe,
    LessThan,
    GreaterThan,
    Equal,
    Tilde,
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "class" => Self::Class,
            "constructor" => Self::Constructor,
            "function" => Self::Function,
            "method" => Self::Method,
            "field" => Self::Field,
            "void" => Self::Void,
            "static" => Self::Static,
            "var" => Self::Var,
            "do" => Self::DO,
            "if" => Self::IF,
            "else" => Self::Else,
            "while" => Self::While,
            "return" => Self::Return,
            "true" => Self::True,
            "false" => Self::False,
            "this" => Self::This,
            "that" => Self::That,
            "char" => Self::Char,
            "int" => Self::Int,
            "boolean" => Self::Boolean,
            "null" => Self::Null,
            _ => return Err(()),
        })
    }
}

impl FromStr for Symbol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "{" => Self::LeftBrace,
            "}" => Self::RightBrace,
            "(" => Self::LeftParen,
            ")" => Self::RightParen,
            "[" => Self::LeftBracket,
            "]" => Self::RightBracket,
            "." => Self::Dot,
            "," => Self::Comma,
            ";" => Self::Semicolon,
            "+" => Self::Plus,
            "-" => Self::Minus,
            "*" => Self::Star,
            "/" => Self::Slash,
            "&" => Self::Ampersign,
            "|" => Self::Pipe,
            "<" => Self::LessThan,
            ">" => Self::GreaterThan,
            "=" => Self::Equal,
            "~" => Self::Tilde,
            _ => return Err(()),
        })
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Class => "class",
            Self::Constructor => "constructor",
            Self::Function => "function",
            Self::Method => "method",
            Self::Field => "field",
            Self::Void => "void",
            Self::Static => "static",
            Self::Var => "var",
            Self::DO => "do",
            Self::IF => "if",
            Self::Else => "else",
            Self::While => "while",
            Self::Let => "let",
            Self::Return => "return",
            Self::True => "true",
            Self::False => "false",
            Self::This => "this",
            Self::That => "that",
            Self::Char => "char",
            Self::Int => "int",
            Self::Boolean => "boolean",
            Self::Null => "null",
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::LeftBrace => "{",
            Self::RightBrace => "}",
            Self::LeftParen => "(",
            Self::RightParen => ")",
            Self::LeftBracket => "[",
            Self::RightBracket => "]",
            Self::Dot => ".",
            Self::Comma => ",",
            Self::Semicolon => ";",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Star => "*",
            Self::Slash => "/",
            Self::Ampersign => "&",
            Self::Pipe => "|",
            Self::LessThan => "<",
            Self::GreaterThan => ">",
            Self::Equal => "=",
            Self::Tilde => "~",
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Keyword(string) => write!(f, "Keyword({string})"),
            TokenType::Symbol(string) => write!(f, "Symbol({string})"),
            TokenType::IntegerConstant(int) => write!(f, "IntegerConstant({int})"),
            TokenType::StringConstant(string) => write!(f, "StringConstant({string})"),
            TokenType::Identifier(string) => write!(f, "Identifier({string})"),
        }
    }
}
