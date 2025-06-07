use std::io::{self, Write};
mod lexer;

fn main() {
    let mut buffer = String::new();

    print!("Insert expression: ");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();

    println!("Input: {}", buffer);

    let _result = lexer::tokenize(&buffer);
}
