use thiserror::Error;

use crate::lexer::{Span, SpannedToken, Token};

#[derive(Debug, Default)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Let(Ident, Expression),
    Expression(Expression),
}

#[derive(Debug)]
pub struct Ident {
    pub name: String,
    pub location: usize,
}

#[derive(Debug)]
pub enum Expression {
    Int {
        literal: i64,
        location: usize,
    },
    Bool {
        literal: bool,
        location: usize,
    },
    String {
        literal: String,
        location: usize,
    },
    Unary {
        operator: PrefixOperator,
        expression: Box<Expression>,
        location: usize,
    },
    Binary {
        operator: InfixOperator,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
        location: usize,
    },
    Assignment {
        ident: Box<Expression>,
        expression: Box<Expression>,
        location: usize,
    },
    If {
        condition: Box<Expression>,
        consequence: Box<Statement>,
        alternative: Box<Statement>,
    },
}

#[derive(Debug)]
pub enum PrefixOperator {
    Not,
    Negation,
}

#[derive(Debug, PartialEq)]
pub enum InfixOperator {
    Add,
    Sub,
    Mul,
    Div,
    Assign,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,
}

#[derive(Debug, Error)]
pub enum PrefixOperatorError {
    #[error("cannot convert {0:?} to a prefix operator")]
    InvalidToken(SpannedToken),
}

#[derive(Debug, Error)]
pub enum InfixOperatorError {
    #[error("cannot convert {0:?} to a infix operator")]
    InvalidToken(SpannedToken),
}

impl Expression {
    pub fn unary(operator: PrefixOperator, expression: Expression, location: usize) -> Self {
        Self::Unary {
            operator,
            expression: Box::new(expression),
            location,
        }
    }

    pub fn binary(
        operator: InfixOperator,
        lhs: Expression,
        rhs: Expression,
        location: usize,
    ) -> Self {
        Self::Binary {
            operator,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            location,
        }
    }
}

impl TryFrom<&SpannedToken> for PrefixOperator {
    type Error = PrefixOperatorError;

    fn try_from(token: &SpannedToken) -> Result<Self, Self::Error> {
        match token.token {
            Token::Bang => Ok(PrefixOperator::Not),
            Token::Minus => Ok(PrefixOperator::Negation),
            _ => Err(PrefixOperatorError::InvalidToken(token.clone())),
        }
    }
}

impl TryFrom<&SpannedToken> for InfixOperator {
    type Error = InfixOperatorError;

    fn try_from(token: &SpannedToken) -> Result<Self, Self::Error> {
        match token.token {
            Token::Plus => Ok(InfixOperator::Add),
            Token::Minus => Ok(InfixOperator::Sub),
            Token::Asterix => Ok(InfixOperator::Mul),
            Token::Slash => Ok(InfixOperator::Div),
            Token::Assign => Ok(InfixOperator::Assign),
            Token::Less => Ok(InfixOperator::Less),
            Token::Greater => Ok(InfixOperator::Greater),
            Token::LessEqual => Ok(InfixOperator::LessEqual),
            Token::GreaterEqual => Ok(InfixOperator::GreaterEqual),
            Token::Equal => Ok(InfixOperator::Equal),
            Token::NotEqual => Ok(InfixOperator::NotEqual),
            _ => Err(InfixOperatorError::InvalidToken(token.clone())),
        }
    }
}
