#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        match input.trim() {
            "exit 0" => exit(),
            "hello" => println!("Hello, world!"),
            _ => invalid_command(input),
        }
    }
}

fn invalid_command(input: String) {
    println!("{}: command not found", input.trim());
}

fn exit() {
    std::process::exit(0);
}
