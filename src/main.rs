extern crate core;

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::io::{Read, Write};

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
    let mut rl = rustyline::DefaultEditor::new().unwrap();
    // let history_file = "/tmp/ccf_hist.txt";
    //
    // if rl.load_history(history_file).is_err() {
    // }

    loop {
        let readline = rl.readline("$ ");
        match readline {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                rl.add_history_entry(line).expect("TODO: panic message");

                let mut lexer = Lexer::new(line);
                let tokens = lexer.lex();
                let mut parser = Parser::new(tokens);
                let node = parser.parse();

                match node {
                    ASTNode::Command {
                        name,
                        args,
                        redirections,
                    } => run_command(name, &args, redirections, &rl),
                    ASTNode::Pipeline(pipeline) => run_pipeline(pipeline),
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
    // rl.save_history(history_file).unwrap()
}

// TODO: write when shell exits

fn run_pipeline(_pipeline: Vec<ASTNode>) {
    todo!("Not implemented")
}

fn run_command(
    name: String,
    args: &[String],
    redirections: Vec<Redirection>,
    editor: &DefaultEditor,
) {
    let (mut input, mut output, mut errput) = util::check_streams(redirections);
    run_command_stream(name, args, &mut input, &mut output, &mut errput, &editor);
}

fn run_command_stream(
    name: String,
    args: &[String],
    _input_stream: &mut dyn Read,
    iostream: &mut dyn Write,
    err_stream: &mut dyn Write,
    editor: &DefaultEditor,
) {
    if let Ok(command) = name.parse::<Command>() {
        match command {
            Command::Exit => exit_cmd(&args),
            Command::Echo => echo_cmd(&args, iostream),
            Command::Type => type_cmd(&args, iostream, err_stream),
            Command::External(path) => external_cmd(path, &args, iostream, err_stream),
            Command::Pwd => pwd_cmd(iostream),
            Command::Cd => cd_cmd(&args, err_stream),
            Command::History => history_cmd(args, iostream, editor),
            Command::Invalid => invalid_cmd(&name, err_stream),
        }
    } else {
        panic!("Error parsing command")
    }
}
