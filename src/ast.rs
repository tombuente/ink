use std::fmt;

use thiserror::Error;

use crate::lexer::Token;

#[derive(Debug, Default)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Let(Spanned<String>, Spanned<Expression>),
    Expression(Spanned<Expression>),
}

#[derive(Debug)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

#[derive(Debug)]
pub enum Expression {
    Int(i64),
    Bool(bool),
    String(String),
    Var(Spanned<String>),
    Unary {
        operator: Spanned<UnaryOperator>,
        expression: Box<Spanned<Expression>>,
    },
    Binary {
        operator: Spanned<BinaryOperator>,
        lhs: Box<Spanned<Expression>>,
        rhs: Box<Spanned<Expression>>,
    },
    Assignment {
        ident: Box<Spanned<Expression>>,
        expression: Box<Spanned<Expression>>,
    },
    If {
        condition: Box<Spanned<Expression>>,
        consequence: Box<Statement>,
        alternative: Box<Statement>,
    },
}

#[derive(Debug)]
pub enum UnaryOperator {
    Not,
    Negation,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
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
    InvalidToken(Spanned<Token>),
}

#[derive(Debug, Error)]
pub enum InfixOperatorError {
    #[error("cannot convert {0:?} to a infix operator")]
    InvalidToken(Spanned<Token>),
}

#[derive(PartialEq, Clone)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Spanned<Expression> {
    pub fn unary(operator: Spanned<UnaryOperator>, expression: Spanned<Expression>) -> Self {
        let start = operator.span.start;
        let end = expression.span.end;

        Self {
            value: Expression::Unary {
                operator,
                expression: Box::new(expression),
            },
            span: Span { start, end },
        }
    }

    pub fn binary(
        operator: Spanned<BinaryOperator>,
        lhs: Spanned<Expression>,
        rhs: Spanned<Expression>,
    ) -> Self {
        let start = lhs.span.start;
        let end = rhs.span.end;

        Self {
            value: Expression::Binary {
                operator,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
            span: Span { start, end },
        }
    }
}

impl TryFrom<&Spanned<Token>> for UnaryOperator {
    type Error = PrefixOperatorError;

    fn try_from(token: &Spanned<Token>) -> Result<Self, Self::Error> {
        match token.value {
            Token::Bang => Ok(UnaryOperator::Not),
            Token::Minus => Ok(UnaryOperator::Negation),
            _ => Err(PrefixOperatorError::InvalidToken(token.clone())),
        }
    }
}

impl TryFrom<&Spanned<Token>> for BinaryOperator {
    type Error = InfixOperatorError;

    fn try_from(token: &Spanned<Token>) -> Result<Self, Self::Error> {
        match token.value {
            Token::Plus => Ok(BinaryOperator::Add),
            Token::Minus => Ok(BinaryOperator::Sub),
            Token::Asterix => Ok(BinaryOperator::Mul),
            Token::Slash => Ok(BinaryOperator::Div),
            Token::Assign => Ok(BinaryOperator::Assign),
            Token::Less => Ok(BinaryOperator::Less),
            Token::Greater => Ok(BinaryOperator::Greater),
            Token::LessEqual => Ok(BinaryOperator::LessEqual),
            Token::GreaterEqual => Ok(BinaryOperator::GreaterEqual),
            Token::Equal => Ok(BinaryOperator::Equal),
            Token::NotEqual => Ok(BinaryOperator::NotEqual),
            _ => Err(InfixOperatorError::InvalidToken(token.clone())),
        }
    }
}

impl<T> Spanned<T> {
    pub fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }
}

impl<T: fmt::Debug> fmt::Debug for Spanned<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_name = std::any::type_name::<T>();
        let name = type_name.rsplit("::").next().unwrap_or(type_name);

        f.debug_struct(name)
            .field("value", &self.value)
            .field("span", &self.span)
            .finish()
    }
}
