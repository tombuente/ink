mod lexer;

fn main() {
    let tokens = lexer::tokenize("let x").unwrap();

    println!("{:#?}", tokens);
}
