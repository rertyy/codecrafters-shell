extern crate core;

use crate::parser::RedirectionType;
use std::io::{self, Read, Write};

mod commands;
mod enums;
mod lexer;
mod parser;
pub mod util;

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
        panic!("Not implemented")
    }

    fn run_command(name: String, args: &[String], redirections: Vec<Redirection>) {
        let (mut input, mut output, mut errput) = check_streams(redirections);
        run_command_stream(name, args, &mut input, &mut output, &mut errput);
    }

    fn check_streams(
        redirection: Vec<Redirection>,
    ) -> (Box<dyn Read>, Box<dyn Write>, Box<dyn Write>) {
        let mut input_stream: Box<dyn Read> = Box::new(io::stdin());
        let mut iostream: Box<dyn Write> = Box::new(io::stdout());
        let mut errstream: Box<dyn Write> = Box::new(io::stderr());

        for redir in redirection {
            match redir.direction {
                RedirectionType::Input => {
                    if let Ok(file) = std::fs::File::open(&redir.target) {
                        input_stream = Box::new(file);
                    } else {
                        writeln!(errstream, "Error opening input file: {}", redir.target).unwrap();
                    }
                }
                RedirectionType::Output => {
                    if let Ok(file) = std::fs::File::create(&redir.target) {
                        iostream = Box::new(file);
                    } else {
                        writeln!(errstream, "Error creating output file: {}", redir.target)
                            .unwrap();
                    }
                }
                RedirectionType::Append => {
                    if let Ok(file) = std::fs::OpenOptions::new().append(true).open(&redir.target) {
                        iostream = Box::new(file);
                    } else {
                        writeln!(errstream, "Error opening append file: {}", redir.target).unwrap();
                    }
                }
            }
        }
        (input_stream, iostream, errstream)
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
                Command::Invalid => invalid_cmd(&name, err_stream),
            }
        } else {
            panic!("Error parsing command")
        }
    }
}
