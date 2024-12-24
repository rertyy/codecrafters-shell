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
        let cmd = input.split_whitespace().collect::<Vec<&str>>();
        match cmd[0] {
            "exit" => exit(cmd[1]),
            "echo" => echo(cmd[1..].join(" ")),
            _ => invalid_command(input),
        }
    }
}

fn invalid_command(input: String) {
    println!("{}: command not found", input.trim());
}

fn echo(msg: String) {
    println!("{}", msg);
}

fn exit(code: &str) {
    let code = code.parse::<i32>().unwrap_or(0);
    std::process::exit(code);
}
