// let x = 5;
// x

use std::vec;

use thiserror::Error;

use crate::{
    ast::{Expression, Ident, PrefixOperator, Program, Statement},
    lexer::{SpannedToken, Token},
};

#[derive(Debug, Default)]
pub struct Parser {
    cursor: vec::IntoIter<SpannedToken>,
    spanned: Option<SpannedToken>,
    errors: Vec<ParseError>,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Precedence {
    Lowest = 0,
    Assign = 1,
    Comparison = 2,
    Sum = 3,
    Product = 4,
    Unary = 5,
    Call = 6,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("expected one of {0:?}, found {1:?}")]
    Expect(Vec<Token>, Option<SpannedToken>),
    // #[error("generic parse error: {0}")]
    // Generic(String),
}

#[allow(dead_code)]
impl Parser {
    pub fn new(tokens: Vec<SpannedToken>) -> Self {
        Self {
            cursor: tokens.into_iter(),
            ..Default::default()
        }
    }

    pub fn parse(&mut self) -> Program {
        self.advance();
        let mut program = Program::default();

        while self.spanned.is_some() {
            match self.parse_statement() {
                Ok(statement) => program.statements.push(statement),
                Err(err) => {
                    self.advance();
                    self.errors.push(err)
                }
            }
        }

        program
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.spanned.take() {
            Some(SpannedToken {
                token: Token::Let,
                span: _,
            }) => {
                self.advance();
                self.parse_let_statement()
            }

            token => {
                self.spanned = token;
                Ok(Statement::Expression(
                    self.parse_expression(Precedence::Lowest)?,
                ))
            }
        }
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParseError> {
        let token = self.spanned.take();
        self.advance();

        let lhs = match token {
            Some(SpannedToken {
                token: Token::Word(literal),
                span,
            }) => Expression::String {
                literal,
                location: span.start,
            },

            Some(SpannedToken {
                token: Token::Bang,
                span,
            }) => Expression::unary(
                PrefixOperator::Not,
                self.parse_expression(Precedence::Unary)?,
                span.start,
            ),

            Some(SpannedToken {
                token: Token::Minus,
                span,
            }) => Expression::unary(
                PrefixOperator::Negation,
                self.parse_expression(Precedence::Unary)?,
                span.start,
            ),

            Some(_) => todo!(),
            None => todo!(),
        };

        Ok(lhs)
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParseError> {
        match self.spanned.take() {
            Some(SpannedToken {
                token: Token::Word(name),
                span,
            }) => Ok(Statement::Let(
                Ident {
                    name,
                    location: span.start,
                },
                self.parse_expression(Precedence::Lowest)?,
            )),
            other => Err(ParseError::Expect(vec![Token::Word("".to_string())], other)),
        }
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        todo!()
    }

    fn parse_ident_expression(&mut self, location: usize) -> Result<Statement, ParseError> {
        todo!()
    }

    fn parse_int_expression(&mut self) -> Result<Statement, ParseError> {
        todo!()
    }

    fn parse_string_expression(&mut self) -> Result<Statement, ParseError> {
        todo!()
    }

    fn parse_bool_expression(&mut self) -> Result<Statement, ParseError> {
        todo!()
    }

    fn parse_bang_expression(&mut self) -> Result<Statement, ParseError> {
        todo!()
    }

    fn peek(&self) -> Option<&SpannedToken> {
        self.spanned.as_ref()
    }

    fn advance(&mut self) {
        self.spanned = self.cursor.next();
    }
}

impl From<&SpannedToken> for Precedence {
    fn from(spanned_token: &SpannedToken) -> Self {
        match spanned_token.token {
            Token::Plus => Self::Sum,
            Token::Minus => Self::Sum,
            Token::Assign => Self::Assign,
            Token::Equal => Self::Comparison,
            Token::NotEqual => Self::Comparison,
            Token::Less => Self::Comparison,
            Token::Greater => Self::Comparison,
            Token::LessEqual => Self::Comparison,
            Token::GreaterEqual => Self::Comparison,
            Token::Asterix => Self::Product,
            Token::Slash => Self::Product,
            Token::LeftParan => Self::Call,
            _ => Self::Lowest,
        }
    }
}
