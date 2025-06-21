use crate::parser::{Redirection, RedirectionType};
use crate::HISTORY_FILE;
use std::fs::File;
use std::io::{BufRead, Read, Write};
use std::path::{Path, PathBuf};
use std::{fmt, io};

#[derive(Debug)]
pub struct PathError;

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Path not found")
    }
}

pub fn check_path(cmd: &str) -> Result<PathBuf, PathError> {
    let path_env = std::env::var("PATH").unwrap();
    let paths = path_env.split(":").collect::<Vec<&str>>();

    for path in paths {
        // let full_path = format!("{}/{}", path, cmd);
        let full_path = Path::new(path).join(cmd);
        let pathbuf = PathBuf::from(full_path);
        if pathbuf.exists() {
            return Ok(pathbuf);
        }
    }

    Err(PathError)
}

pub fn check_streams(
    redirection: Vec<Redirection>,
) -> (Box<dyn Read>, Box<dyn Write>, Box<dyn Write>) {
    let mut input_stream: Box<dyn Read> = Box::new(io::stdin());
    let mut iostream: Box<dyn Write> = Box::new(io::stdout());
    let mut errstream: Box<dyn Write> = Box::new(io::stderr());

    for Redirection {
        fd,
        direction,
        target,
    } in redirection
    {
        match (fd, direction) {
            (0, RedirectionType::Input) => {
                todo!("Input")
            }
            (1, RedirectionType::Output) => {
                if let Ok(file) = std::fs::File::create(&target) {
                    iostream = Box::new(file);
                } else {
                    writeln!(errstream, "Error opening input file: {}", target).unwrap();
                }
            }
            (2, RedirectionType::Output) => {
                if let Ok(file) = std::fs::File::create(&target) {
                    errstream = Box::new(file);
                } else {
                    writeln!(errstream, "Error opening input file: {}", target).unwrap();
                }
            }
            (1, RedirectionType::Append) => {
                if let Ok(file) = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&target)
                {
                    iostream = Box::new(file);
                } else {
                    writeln!(errstream, "Error opening append file: {}", target).unwrap();
                }
            }
            (2, RedirectionType::Append) => {
                if let Ok(file) = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&target)
                {
                    errstream = Box::new(file);
                } else {
                    writeln!(errstream, "Error opening append file: {}", target).unwrap();
                }
            }
            _ => unreachable!(),
        }
    }
    (input_stream, iostream, errstream)
}

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_history() {
    if let Ok(lines) = read_lines(HISTORY_FILE) {
        let history: Vec<String> = lines.map_while(Result::ok).collect();
    }
}
pub fn write_history(input: &str) {
    if let Ok(mut history_file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(HISTORY_FILE)
    {
        writeln!(history_file, "{}", input).unwrap();
    } else {
        eprintln!("History file not found");
    }
}
