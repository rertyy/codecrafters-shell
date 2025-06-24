use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;

use crate::util::check_path;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Pipe,
    Output(Option<u8>), // >, 1>, 2>
    Append(Option<u8>), // >>, 1>>, 2>>
    Input,              // <
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(op: &str) -> Result<Self, Self::Err> {
        match op {
            "|" => Ok(Self::Pipe),
            ">" => Ok(Self::Output(None)),
            "1>" => Ok(Self::Output(Some(1))),
            "2>" => Ok(Self::Output(Some(2))),
            ">>" => Ok(Self::Append(None)),
            "1>>" => Ok(Self::Append(Some(1))),
            "2>>" => Ok(Self::Append(Some(2))),
            "<" => Ok(Self::Input),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Exit,
    Echo,
    Type,
    Invalid,
    External(PathBuf),
    Pwd,
    Cd,
    History,
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Exit => "exit",
            Self::Echo => "echo",
            Self::Type => "type",
            Self::Invalid => "invalid",
            Self::External(path) => path.to_str().unwrap_or("invalid"),
            Self::Pwd => "pwd",
            Self::Cd => "cd",
            Self::History => "history",
        };
        write!(f, "{}", str)
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(cmd: &str) -> Result<Self, Self::Err> {
        let result = match cmd {
            "exit" => Self::Exit,
            "echo" => Self::Echo,
            "type" => Self::Type,
            "pwd" => Self::Pwd,
            "cd" => Self::Cd,
            "history" => Self::History,
            _ => check_path(cmd).map(Self::External).unwrap_or(Self::Invalid),
        };
        Ok(result)
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Word(String),
    Operator(Operator),
}

impl FromStr for Token {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Token::Word(s.to_string()))
    }
}
