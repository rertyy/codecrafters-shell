#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::PathBuf;

use crate::util::check_path;

pub enum Command {
    Exit,
    Echo,
    Type,
    Invalid,
    External(PathBuf),
    Pwd,
    Cd,
}

impl Command {
    // TODO: figure out how to impl trait FromStr
    pub fn parse_str(cmd: &str) -> Self {
        match cmd {
            "exit" => Self::Exit,
            "echo" => Self::Echo,
            "type" => Self::Type,
            "pwd" => Self::Pwd,
            "cd" => Self::Cd,
            _ => check_path(cmd).map(Self::External).unwrap_or(Self::Invalid),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Exit => "exit",
            Self::Echo => "echo",
            Self::Type => "type",
            Self::Invalid => "invalid",
            Self::External(p) => p.file_name().unwrap().to_str().unwrap(),
            Self::Pwd => "pwd",
            Self::Cd => "cd",
        }
    }
}
