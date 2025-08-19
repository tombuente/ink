use std::{error::Error, vec};

use thiserror::Error;

use crate::{
    ast::{Expression, Program, Statement},
    lexer::SpannedToken,
};

#[derive(Debug, Default)]
pub struct Parser {
    cursor: vec::IntoIter<SpannedToken>,
    token: Option<SpannedToken>,
    errors: Vec<ParseError>,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("generic parse error: {0}")]
    Generic(String),
}

impl Parser {
    pub fn new(tokens: Vec<SpannedToken>) -> Self {
        Self {
            cursor: tokens.into_iter(),
            ..Default::default()
        }
    }

    pub fn parse(&mut self) -> Program {
        self.next();
        let mut program = Program::default();

        while self.token.is_some() {
            match self.parse_statement() {
                Ok(statement) => program.statements.push(statement),
                Err(err) => {
                    self.next();
                    self.errors.push(err)
                }
            }
        }

        program
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        Ok(Statement::Expression(self.parse_expression()?))
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        Ok(Expression::Ident("word".to_string()))
    }

    fn next(&mut self) {
        self.token = self.cursor.next();
    }
}
