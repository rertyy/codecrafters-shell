#[allow(unused_imports)]
use std::io::{self, Write};

pub mod commands;
pub mod enums;
pub mod util;

use crate::commands::*;
use crate::enums::Command;

fn main() {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input_vec = util::parse_input(&input);
        if input_vec.is_empty() {
            continue;
        }

        let command = Command::parse_str(input_vec[0]);
        match command {
            Command::Exit => exit_cmd(input_vec[1]),
            Command::Echo => echo_cmd(&input_vec[1..].join(" ")),
            Command::Type => type_cmd(input_vec[1]),
            Command::External(path) => external_cmd(path, input_vec[1..].to_vec()),
            Command::Invalid => invalid_cmd(&input),
            Command::Pwd => pwd_cmd(),
            Command::Cd => cd_cmd(input_vec[1]),
        }
    }
}
