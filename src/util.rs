use crate::parser::{Redirection, RedirectionType};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{BufRead, Read, Write};
use std::os::unix::fs::PermissionsExt;
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
        if full_path.is_file() {
            let metadata = full_path.metadata().ok();
            if let Some(meta) = metadata {
                if meta.permissions().mode() & 0o111 != 0 {
                    return Ok(full_path);
                }
            }
        }
    }

    Err(PathError)
}

pub fn get_path_executables() -> HashMap<String, PathBuf> {
    // this is very slow...
    let mut seen = HashSet::new();

    std::env::var("PATH")
        .unwrap_or_default()
        .split(':')
        .flat_map(|dir| fs::read_dir(dir).into_iter().flat_map(|it| it.flatten()))
        .filter_map(|entry| {
            let path = entry.path();
            let name = path.file_name()?.to_str()?.to_string();
            if path.is_file() && seen.insert(name.clone()) {
                Some((name, path))
            } else {
                None
            }
        })
        .collect()
}

pub fn get_path_exe_strings(exe: HashMap<String, PathBuf>) -> Vec<String> {
    exe.iter().map(|(name, _)| name.to_string()).collect()
}

fn create_file(target: &str) -> Result<Box<dyn Write>, io::Error> {
    let file = File::create(target)?;
    Ok(Box::new(file))
}

fn append_file(target: &str) -> Result<Box<dyn Write>, io::Error> {
    let file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&target)?;
    Ok(Box::new(file))
}

pub fn check_streams(
    redirection: Vec<Redirection>,
) -> (Box<dyn Read>, Box<dyn Write>, Box<dyn Write>) {
    let input_stream: Box<dyn Read> = Box::new(io::stdin());
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
                iostream = create_file(&target).unwrap();
            }
            (2, RedirectionType::Output) => {
                errstream = create_file(&target).unwrap();
            }
            (1, RedirectionType::Append) => {
                iostream = append_file(&target).unwrap();
            }
            (2, RedirectionType::Append) => {
                errstream = append_file(&target).unwrap();
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

pub fn read_history(history_file: &str) -> Vec<String> {
    if let Ok(lines) = read_lines(history_file) {
        let history: Vec<String> = lines.map_while(Result::ok).collect();
        history
    } else {
        Vec::new()
    }
}

pub fn append_history(history: &[String], history_file: &str) {
    if let Ok(mut file_ref) = fs::OpenOptions::new()
        .append(true)
        .write(true)
        .open(history_file)
    {
        for entry in history {
            writeln!(file_ref, "{}", entry).unwrap();
        }
    }
}
pub fn write_history(history: &[String], history_file: &str) {
    if let Ok(mut file_ref) = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(history_file)
    {
        for entry in history {
            writeln!(file_ref, "{}", entry).unwrap();
        }
    }
}
