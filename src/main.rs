#[allow(unused_imports)]
use std::io::{self, Write};
use std::{path::PathBuf, process};

enum Command {
    Exit,
    Echo,
    Type,
    Invalid,
    External(PathBuf),
    Pwd,
}

impl Command {
    fn from_str(cmd: &str) -> Self {
        match cmd {
            "exit" => Self::Exit,
            "echo" => Self::Echo,
            "type" => Self::Type,
            "pwd" => Self::Pwd,
            _ => check_path(cmd).map(Self::External).unwrap_or(Self::Invalid),
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Self::Exit => "exit",
            Self::Echo => "echo",
            Self::Type => "type",
            Self::Invalid => "invalid",
            Self::External(p) => p.file_name().unwrap().to_str().unwrap(),
            Self::Pwd => "pwd",
        }
    }
}

fn check_path(cmd: &str) -> Result<PathBuf, ()> {
    let path_env = std::env::var("PATH").unwrap();
    let paths = path_env.split(":").collect::<Vec<&str>>();

    for path in paths {
        let full_path = format!("{}/{}", path, cmd);
        let pathbuf = PathBuf::from(full_path);
        if pathbuf.exists() {
            return Ok(pathbuf);
        }
    }

    Err(())
}

fn main() {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input_vec = input.split_whitespace().collect::<Vec<&str>>();
        if input_vec.is_empty() {
            continue;
        }

        let command = Command::from_str(input_vec[0]);
        match command {
            Command::Exit => exit_cmd(input_vec[1]),
            Command::Echo => echo_cmd(&input_vec[1..].join(" ")),
            Command::Type => type_cmd(input_vec[1]),
            Command::External(path) => external_cmd(path, input_vec[1..].to_vec()),
            Command::Invalid => invalid_cmd(&input),
            Command::Pwd => pwd_cmd(),
        }
    }
}

fn pwd_cmd() {
    let current_dir = std::env::current_dir().unwrap();
    println!("{}", current_dir.display());
}

fn external_cmd(path: PathBuf, input: Vec<&str>) {
    match process::Command::new(path).args(input).output() {
        Ok(output) => {
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        }
        Err(e) => eprintln!("{}", e),
    }
}

fn invalid_cmd(input: &str) {
    println!("{}: command not found", input.trim());
}

fn type_cmd(input: &str) {
    let cmd = Command::from_str(input);

    match cmd {
        Command::External(path) => println!("{} is {}", input, path.to_str().unwrap()),
        Command::Invalid => println!("{}: not found", input),
        Command::Exit | Command::Echo | Command::Type | Command::Pwd => {
            println!("{} is a shell builtin", cmd.as_str())
        }
    }
}

fn echo_cmd(msg: &str) {
    println!("{}", msg);
}

fn exit_cmd(code: &str) {
    let code = code.parse::<i32>().unwrap_or(0);
    std::process::exit(code);
}
