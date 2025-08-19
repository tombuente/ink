use thiserror::Error;

use crate::lexer::{SpannedToken, Token};

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
            Token::Ident(_) => todo!(),
            _ => Err(InfixOperatorError::InvalidToken(token.clone())),
        }
    }
}
