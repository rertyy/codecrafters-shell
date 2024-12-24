#[allow(unused_imports)]
use std::io::{self, Write};

enum Command {
    Exit,
    Echo,
    Type,
    Invalid,
}

impl Command {
    fn from_str(cmd: &str) -> Self {
        match cmd {
            "exit" => Self::Exit,
            "echo" => Self::Echo,
            "type" => Self::Type,
            _ => Self::Invalid,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Self::Exit => "exit",
            Self::Echo => "echo",
            Self::Type => "type",
            Self::Invalid => "invalid",
        }
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input_vec = input.split_whitespace().collect::<Vec<&str>>();
        let command = Command::from_str(input_vec[0]);
        match command {
            Command::Exit => exit(input_vec[1]),
            Command::Echo => echo(input_vec[1..].join(" ")),
            Command::Invalid => invalid_command(input),
            Command::Type => type_cmd(input_vec[1]),
        }
    }
}

fn invalid_command(input: String) {
    println!("{}: command not found", input.trim());
}

fn type_cmd(input: &str) {
    let cmd = Command::from_str(input);
    match cmd {
        Command::Invalid => println!("{}: not found", input),
        _ => println!("{} is a shell builtin", cmd.as_str()),
    }
}

fn echo(msg: String) {
    println!("{}", msg);
}

fn exit(code: &str) {
    let code = code.parse::<i32>().unwrap_or(0);
    std::process::exit(code);
}
