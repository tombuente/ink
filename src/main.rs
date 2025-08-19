mod ast;
mod lexer;
mod parser;

fn main() {
    let tokens = lexer::tokenize("let x").unwrap();

    println!("{:#?}", tokens);
}
