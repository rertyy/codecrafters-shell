use std::fmt;
use std::path::PathBuf;

use regex::Regex;

#[derive(Debug)]
pub struct PathError;

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Path not found")
    }
}

pub fn parse_input(cmd: &str) -> Vec<&str> {
    let re = Regex::new(r"'[^']+'|\S+").unwrap();
    re.find_iter(cmd)
        .map(|m| m.as_str())
        .map(|s| s.trim_matches('\''))
        .collect()
}

pub fn check_path(cmd: &str) -> Result<PathBuf, PathError> {
    let path_env = std::env::var("PATH").unwrap();
    let paths = path_env.split(":").collect::<Vec<&str>>();

    for path in paths {
        let full_path = format!("{}/{}", path, cmd);
        let pathbuf = PathBuf::from(full_path);
        if pathbuf.exists() {
            return Ok(pathbuf);
        }
    }

    Err(PathError)
}
