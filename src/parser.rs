use std::vec;

use thiserror::Error;

use crate::{
    ast::{BinaryOperator, Expression, Program, Span, Spanned, Statement, UnaryOperator},
    lexer::Token,
};

#[derive(Debug, Default)]
pub struct Parser<'a> {
    source: &'a str,
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
}

const EXPR_START: &[Token] = &[
    Token::Minus,
    Token::Bang,
    Token::True,
    Token::False,
    Token::Literal,
];

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Spanned<Token>>, source: &'a str) -> Self {
        Self {
            source,
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
        let token = self.expect(EXPR_START)?;

        match token {
            Spanned {
                value: Token::Let,
                span: _,
            } => self.parse_let_statement(),

            token => {
                self.token = Some(token);

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
        let token = self.expect(&[])?;
        let mut lhs = match token {
            Spanned {
                value: Token::Minus,
                span,
            } => Spanned::unary(
                Spanned::new(UnaryOperator::Negation, span),
                self.parse_expression(Precedence::Unary)?,
            ),

            Spanned {
                value: Token::Bang,
                span,
            } => Spanned::unary(
                Spanned::new(UnaryOperator::Not, span),
                self.parse_expression(Precedence::Unary)?,
            ),

            Spanned {
                value: Token::True,
                span,
            } => Spanned::new(Expression::Bool(Spanned::new(true, span)), span),

            Spanned {
                value: Token::False,
                span,
            } => Spanned::new(Expression::Bool(Spanned::new(false, span)), span),

            Spanned {
                value: Token::Literal,
                span,
            } => Spanned::new(
                Expression::Var(Spanned::new(self.literal(span).into(), span)),
                span,
            ),

            _ => {
                return Err(ParseError::Expect(EXPR_START.into(), Some(token)));
            }
        };

        while precedence
            < self
                .peek()
                .map_or(Precedence::Lowest, |token| Precedence::of(&token.value))
        {
            let token = self.expect(&[])?;
            let precedence = Precedence::of(&token.value);

            let operator = match token {
                Spanned {
                    value: Token::Plus,
                    span,
                } => Spanned::new(BinaryOperator::Add, span),
                Spanned {
                    value: Token::Minus,
                    span,
                } => Spanned::new(BinaryOperator::Sub, span),
                Spanned {
                    value: Token::Asterisk,
                    span,
                } => Spanned::new(BinaryOperator::Mul, span),
                Spanned {
                    value: Token::Slash,
                    span,
                } => Spanned::new(BinaryOperator::Div, span),
                Spanned {
                    value: Token::Less,
                    span,
                } => Spanned::new(BinaryOperator::Less, span),
                Spanned {
                    value: Token::Greater,
                    span,
                } => Spanned::new(BinaryOperator::Greater, span),
                Spanned {
                    value: Token::LessEqual,
                    span,
                } => Spanned::new(BinaryOperator::LessEqual, span),
                Spanned {
                    value: Token::GreaterEqual,
                    span,
                } => Spanned::new(BinaryOperator::GreaterEqual, span),
                Spanned {
                    value: Token::Equal,
                    span,
                } => Spanned::new(BinaryOperator::Equal, span),
                Spanned {
                    value: Token::NotEqual,
                    span,
                } => Spanned::new(BinaryOperator::NotEqual, span),
                Spanned {
                    value: Token::Assign,
                    span: _,
                } => todo!(),
                _ => {
                    return Err(ParseError::Expect(
                        vec![
                            Token::Plus,
                            Token::Minus,
                            Token::Asterisk,
                            Token::Slash,
                            Token::Less,
                            Token::Greater,
                            Token::LessEqual,
                            Token::GreaterEqual,
                            Token::Equal,
                            Token::NotEqual,
                            Token::Assign,
                        ],
                        Some(token),
                    ));
                }
            };

            let rhs = self.parse_expression(precedence)?;

            lhs = Spanned::binary(operator, lhs, rhs)
        }

        Ok(lhs)
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParseError> {
        todo!()
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

    fn expect(&mut self, take: &[Token]) -> Result<Spanned<Token>, ParseError> {
        let token = self
            .token
            .take()
            .ok_or(ParseError::Expect(take.to_vec(), None))?;

        if !take.contains(&token.value) {
            self.advance();
        }

        Ok(token)
    }

    fn advance(&mut self) {
        self.token = self.cursor.next();
    }

    fn literal(&self, span: Span) -> &str {
        &self.source[span.start..span.end]
    }
}

impl Precedence {
    fn of(token: &Token) -> Self {
        match token {
            Token::Plus | Token::Minus => Self::Sum,
            Token::Assign => Self::Assign,
            Token::Equal
            | Token::NotEqual
            | Token::Less
            | Token::Greater
            | Token::LessEqual
            | Token::GreaterEqual => Self::Comparison,
            Token::Asterisk | Token::Slash => Self::Product,
            Token::LeftParan => Self::Call,
            _ => Self::Lowest,
        }
    }
}
