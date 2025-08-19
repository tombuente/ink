use crate::ast::{InfixOperator, PrefixOperator};

#[derive(Debug)]
pub enum Expression {
    Ident(String),
    Prefix {
        operator: PrefixOperator,
        expression: Box<Expression>,
    },
    Infix {
        operator: InfixOperator,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
}

impl Expression {
    pub fn prefix(operator: PrefixOperator, expression: Expression) -> Self {
        Self::Prefix {
            operator,
            expression: Box::new(expression),
        }
    }

    pub fn infix(operator: InfixOperator, lhs: Expression, rhs: Expression) -> Self {
        Self::Infix {
            operator,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}
