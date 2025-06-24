use std::io::{self, Write};
mod lexer;
mod parser;

fn main() {
    let mut buffer = String::new();

    print!("Insert expression: ");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();

    println!("Input: {}", buffer);

    let tokens = lexer::tokenize(&buffer);
    let _result = parser::parse(&tokens.unwrap()); // TODO: add match case for error handling
}
