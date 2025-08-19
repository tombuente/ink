use crate::ast::Expression;

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
}

impl Statement {
    pub fn from_expression(expression: Expression) -> Self {
        Statement::Expression(expression)
    }
}
