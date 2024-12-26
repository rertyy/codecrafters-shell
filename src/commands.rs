#[allow(unused_imports)]
use std::io::{self, Write};
use std::{path::PathBuf, process};

use crate::enums::Command;

pub fn cd_cmd(dir: &str) {
    let path = PathBuf::from(dir);
    let home = std::env::var("HOME").unwrap();
    let replaced = path.display().to_string().replace("~", &home);
    if std::env::set_current_dir(&replaced).is_err() {
        eprintln!("cd: {}: No such file or directory", dir);
    }
}

pub fn pwd_cmd() {
    let current_dir = std::env::current_dir().unwrap();
    println!("{}", current_dir.display());
}

pub fn external_cmd(path: PathBuf, input: Vec<String>) {
    match process::Command::new(path).args(input).output() {
        Ok(output) => {
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        }
        Err(e) => eprintln!("{}", e),
    }
}

pub fn invalid_cmd(input: &str) {
    println!("{}: command not found", input.trim());
}

pub fn type_cmd(input: &str) {
    let cmd = Command::parse_str(input);

    match cmd {
        Command::External(path) => println!("{} is {}", input, path.to_str().unwrap()),
        Command::Invalid => println!("{}: not found", input),
        Command::Exit | Command::Echo | Command::Type | Command::Pwd | Command::Cd => {
            println!("{} is a shell builtin", cmd.as_str())
        }
    }
}
pub fn echo_cmd(msg: &str) {
    println!("{}", msg);
}

pub fn exit_cmd(code: &str) {
    let code = code.parse::<i32>().unwrap_or(0);
    std::process::exit(code);
}
