use crate::parser::Parser;

mod ast;
mod lexer;
mod parser;

fn main() {
    let source = "x";
    let tokens = lexer::tokenize(source).unwrap();

    println!("{:#?}", tokens);

    let mut parser = Parser::new(tokens, source);
    let program = parser.parse();

    println!("{:#?}", program);
}
