use crate::ast::Statement;

#[derive(Debug, Default)]
pub struct Program {
    pub statements: Vec<Statement>,
}
