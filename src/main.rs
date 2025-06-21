extern crate core;

use std::io::{self, Read, Write};

mod commands;
mod enums;
mod lexer;
mod parser;
pub mod util;

const HISTORY_FILE: &str = "/tmp/ccf_hist.txt";

use crate::commands::*;
use crate::enums::Command;
use crate::lexer::Lexer;
use crate::parser::{ASTNode, Parser, Redirection};

fn main() {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        input = input.trim().to_string();
        if input.is_empty() {
            continue;
        }

        let mut lexer = Lexer::new(&*input);
        let tokens = lexer.lex();
        // println!("{:?}", tokens);

        let mut parser = Parser::new(tokens);
        let node = parser.parse();
        // println!("{:?}", node);

        util::write_history(&input);

        match node {
            ASTNode::Command {
                name,
                args,
                redirections,
            } => run_command(name, &args, redirections),
            ASTNode::Pipeline(pipeline) => run_pipeline(pipeline),
        }
    }

    fn run_pipeline(pipeline: Vec<ASTNode>) {
        todo!("Not implemented")
    }

    fn run_command(name: String, args: &[String], redirections: Vec<Redirection>) {
        let (mut input, mut output, mut errput) = util::check_streams(redirections);
        run_command_stream(name, args, &mut input, &mut output, &mut errput);
    }

    fn run_command_stream(
        name: String,
        args: &[String],
        input_stream: &mut dyn Read,
        iostream: &mut dyn Write,
        err_stream: &mut dyn Write,
    ) {
        if let Ok(command) = name.parse::<Command>() {
            match command {
                Command::Exit => exit_cmd(&args),
                Command::Echo => echo_cmd(&args, iostream),
                Command::Type => type_cmd(&args, iostream, err_stream),
                Command::External(path) => external_cmd(path, &args, iostream, err_stream),
                Command::Pwd => pwd_cmd(iostream),
                Command::Cd => cd_cmd(&args, err_stream),
                Command::History => history_cmd(iostream),
                Command::Invalid => invalid_cmd(&name, err_stream),
            }
        } else {
            panic!("Error parsing command")
        }
    }
}
