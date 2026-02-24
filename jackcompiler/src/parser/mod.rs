mod ast;

use crate::error::ParseError;
use crate::lexer::token::Token;
use crate::lexer::token::TokenType;
use crate::parser::ast::Expr;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    // term: varName | constant
    fn term(&mut self) -> Result<Expr, ParseError> {
        match &self.current_token().token_type {
            TokenType::IntegerConstant(n) => {
                let n = *n;
                self.advance();
                Ok(Expr::IntegerConstant(n))
            }
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(Expr::Variable(name))
            }
            _ => Err(self.error("expected term")),
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous_token()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn current_token(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous_token(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        matches!(
            self.tokens.get(self.current),
            None | Some(Token {
                token_type: TokenType::EOF,
                ..
            })
        )
    }

    fn match_any(&mut self, token_types: &[TokenType]) -> bool {
        if token_types.iter().any(|&t| self.check(t)) {
            self.advance();
            return true;
        }
        false
    }
}
