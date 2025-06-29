extern crate core;

use rustyline::error::ReadlineError;
use rustyline::history::{DefaultHistory, History};
use rustyline::{CompletionType, Editor};
use std::io::{Read, Write};

mod commands;
mod completer;
mod enums;
mod lexer;
mod parser;
pub mod util;

use crate::commands::*;
use crate::completer::MyHelper;
use crate::enums::Command;
use crate::lexer::Lexer;
use crate::parser::{ASTNode, Parser, Redirection};

fn main() -> rustyline::Result<()> {
    let helper = MyHelper {};

    let config = rustyline::Config::builder()
        .history_ignore_dups(false)?
        .completion_type(CompletionType::List)
        .build();
    let mut rl = Editor::<MyHelper, DefaultHistory>::with_config(config)?;
    rl.set_helper(Some(helper));

    // let executables = util::get_path_executables();

    let history_file = std::env::var("HISTFILE").unwrap_or_default();

    let _ = rl.load_history(&history_file);

    let mut last_saved_history_idx = rl.history().len();

    loop {
        let readline = rl.readline("$ ");
        match readline {
            Ok(line) => {
                // println!("DEBUG: {:?}", line);
                let line = line.trim_start();
                if line.is_empty() {
                    continue;
                }

                rl.add_history_entry(line).expect("TODO: panic message");

                let mut lexer = Lexer::new(line);
                let tokens = lexer.lex();
                // println!("{:#?}", tokens);
                let mut parser = Parser::new(tokens);
                let node = parser.parse();

                match node {
                    ASTNode::Command {
                        name,
                        args,
                        redirections,
                    } => run_command(
                        name,
                        &args,
                        redirections,
                        &mut rl,
                        &mut last_saved_history_idx,
                    ),
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
    // rl.append_history(&history_file)?;
    let _ = rl.save_history(&history_file);
    Ok(())
}

fn run_pipeline(_pipeline: Vec<ASTNode>) {
    todo!("Not implemented")
}

fn run_command(
    name: String,
    args: &[String],
    redirections: Vec<Redirection>,
    editor: &mut Editor<MyHelper, DefaultHistory>,
    last_saved_history_idx: &mut usize,
) {
    let (mut input, mut output, mut errput) = util::check_streams(redirections);
    run_command_stream(
        name,
        args,
        &mut input,
        &mut output,
        &mut errput,
        editor,
        last_saved_history_idx,
    );
}

fn run_command_stream(
    name: String,
    args: &[String],
    _input_stream: &mut dyn Read,
    iostream: &mut dyn Write,
    err_stream: &mut dyn Write,
    editor: &mut Editor<MyHelper, DefaultHistory>,
    last_saved_history_idx: &mut usize,
) {
    if let Ok(command) = name.parse::<Command>() {
        match command {
            Command::Exit => exit_cmd(&args, editor, last_saved_history_idx),
            Command::Echo => echo_cmd(&args, iostream),
            Command::Type => type_cmd(&args, iostream, err_stream),
            Command::External(path) => external_cmd(path, &args, iostream, err_stream),
            Command::Pwd => pwd_cmd(iostream),
            Command::Cd => cd_cmd(&args, err_stream),
            Command::History => history_cmd(args, iostream, editor, last_saved_history_idx),
            Command::Invalid => invalid_cmd(&name, err_stream),
        }
    } else {
        panic!("Error parsing command")
    }
}
