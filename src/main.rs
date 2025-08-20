use crate::parser::Parser;

mod ast;
mod lexer;
mod parser;

fn main() {
    let tokens = lexer::tokenize("!x").unwrap();

    println!("{:#?}", tokens);

    let mut parser = Parser::new(tokens);
    let program = parser.parse();

    println!("{:#?}", program);
}
