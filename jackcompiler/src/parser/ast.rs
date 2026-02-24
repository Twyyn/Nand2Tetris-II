use crate::lexer::token::Token;
use std::fmt::Display;

pub type Identifier = String;

#[derive(Debug)]
pub struct Class {
    name: Identifier,
    variables: Vec<ClassVarDeclaration>,
}

#[derive(Debug)]
pub struct ClassVarDeclaration {
    pub class_type: ClassVarType,
    pub var_type: Type,
    pub names: Vec<Identifier>,
}

#[derive(Debug)]
pub struct VarDeclaration {
    pub var_type: Type,
    pub names: Vec<Identifier>,
}

#[derive(Debug)]
pub struct SubroutineDeclaration {
    subroutine_type: SubroutineType,
    ruturn_type: ReturnType,
    name: Identifier,
    pub parameters: Vec<(Type, Identifier)>,
    body: SubroutineBody,
}

#[derive(Debug)]
pub struct SubroutineCall {
    pub receiver: Option<Identifier>,
    pub name: Identifier,
    pub arguments: Vec<Expr>,
}

#[derive(Debug)]
pub struct SubroutineBody {
    pub vars: Vec<VarDeclaration>,
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Let {
        name: Identifier,
        expression: Expr,
    },
    If {
        expression: Expr,
        if_statements: Vec<Statement>,
        else_statements: Option<Vec<Statement>>,
    },
    While {
        expression: Expr,
        statements: Vec<Statement>,
    },
    Do(SubroutineCall),
    Return(Option<Expr>),
}

#[derive(Debug)]
pub enum Expr {
    IntegerConstant(u16),
    StringConstant(String),
    Unary {
        operation: UnaryOP,
        expr: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operation: BinaryOP,
        right: Box<Expr>,
    },
    Grouped(Box<Expr>),
    Variable(Token),
}

#[derive(Debug)]
pub enum ClassVarType {
    Static,
    Field,
}

#[derive(Debug)]
pub enum Type {
    Int,
    Char,
    Boolean,
    ClassName(Identifier),
}

#[derive(Debug)]
pub enum SubroutineType {
    Constructor,
    Function,
    Method,
}

#[derive(Debug)]
pub enum ReturnType {
    Void,
    Type(Type),
}

#[derive(Debug)]
pub enum KeywordConst {
    True,
    False,
    Null,
    This,
}

#[derive(Debug)]
pub enum UnaryOP {
    Neg,
    Not,
}

#[derive(Debug)]
pub enum BinaryOP {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Lt,
    Gt,
    Eq,
}
