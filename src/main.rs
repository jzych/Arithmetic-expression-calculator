use std::io::{self, Write};

fn main() {
    let mut buffer = String::new();

    print!("Insert expression: ");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();

    println!("Input: ,{}", buffer);
}
