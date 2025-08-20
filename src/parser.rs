use std::vec;

use thiserror::Error;

use crate::{
    ast::{Expression, Program, Spanned, Statement, UnaryOperator},
    lexer::Token,
};

#[derive(Debug, Default)]
pub struct Parser {
    cursor: vec::IntoIter<Spanned<Token>>,
    token: Option<Spanned<Token>>,
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
    Expect(Vec<Token>, Option<Spanned<Token>>),
    // #[error("generic parse error: {0}")]
    // Generic(String),
}

#[allow(dead_code)]
impl Parser {
    pub fn new(tokens: Vec<Spanned<Token>>) -> Self {
        Self {
            cursor: tokens.into_iter(),
            ..Default::default()
        }
    }

    pub fn parse(&mut self) -> Program {
        self.advance();
        let mut program = Program::default();

        while self.token.is_some() {
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
        match self.token.take() {
            Some(Spanned {
                value: Token::Let,
                span: _,
            }) => {
                self.advance();
                self.parse_let_statement()
            }

            token => {
                self.token = token;

                Ok(Statement::Expression(
                    self.parse_expression(Precedence::Lowest)?,
                ))
            }
        }
    }

    fn parse_expression(
        &mut self,
        precedence: Precedence,
    ) -> Result<Spanned<Expression>, ParseError> {
        let token = self.token.take();
        self.advance();

        let lhs = match token {
            Some(Spanned {
                value: Token::Ident(name),
                span,
            }) => Spanned::new(Expression::Var(Spanned::new(name, span.clone())), span),

            Some(Spanned {
                value: Token::Bang,
                span,
            }) => Spanned::unary(
                Spanned {
                    value: UnaryOperator::Not,
                    span,
                },
                self.parse_expression(Precedence::Unary)?,
            ),

            Some(Spanned {
                value: Token::Minus,
                span,
            }) => Spanned::unary(
                Spanned {
                    value: UnaryOperator::Negation,
                    span,
                },
                self.parse_expression(Precedence::Unary)?,
            ),

            Some(_) => todo!(),
            None => todo!(),
        };

        Ok(lhs)
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParseError> {
        match self.token.take() {
            Some(Spanned {
                value: Token::Ident(name),
                span,
            }) => Ok(Statement::Let(
                Spanned::new(name, span),
                self.parse_expression(Precedence::Lowest)?,
            )),
            other => Err(ParseError::Expect(
                vec![Token::Ident("".to_string())],
                other,
            )),
        }
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        todo!()
    }

    fn parse_ident_expression(&mut self) -> Result<Statement, ParseError> {
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

    fn peek(&self) -> Option<&Spanned<Token>> {
        self.token.as_ref()
    }

    fn advance(&mut self) {
        self.token = self.cursor.next();
    }
}

impl From<&Spanned<Token>> for Precedence {
    fn from(token: &Spanned<Token>) -> Self {
        match token.value {
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
