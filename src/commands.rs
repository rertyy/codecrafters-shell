use std::io::Write;
use std::{path::PathBuf, process};

use crate::enums::Command;
use std::os::unix::process::CommandExt;

pub fn history_cmd(iostream: &mut dyn Write) {
    todo!()
}
pub fn cd_cmd(args: &[String], err_stream: &mut dyn Write) {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/".into());
    let dir = args.get(0).unwrap_or(&home);
    let path = if dir.starts_with('~') {
        PathBuf::from(dir.replacen('~', &home, 1))
    } else {
        PathBuf::from(dir)
    };

    if std::env::set_current_dir(&path).is_err() {
        writeln!(err_stream, "cd: {}: No such file or directory", dir).unwrap();
    }
}

pub fn pwd_cmd(iostream: &mut dyn Write) {
    let current_dir = std::env::current_dir().unwrap();
    writeln!(iostream, "{}", current_dir.display()).unwrap();
}

pub fn external_cmd(
    path: PathBuf,
    args: &[String],
    iostream: &mut dyn Write,
    err_stream: &mut dyn Write,
) {
    let file_name = path.file_name().unwrap_or_default().to_os_string();

    match process::Command::new(path)
        .arg0(file_name)
        .args(args)
        .output()
    {
        Ok(output) => {
            iostream.write_all(&output.stdout).unwrap();
            err_stream.write_all(&output.stderr).unwrap();
        }
        Err(e) => writeln!(err_stream, "Error: {}", e).unwrap(),
    }
}

pub fn invalid_cmd(name: &str, err_stream: &mut dyn Write) {
    writeln!(err_stream, "{}: command not found", name).unwrap();
}

pub fn type_cmd(args: &[String], iostream: &mut dyn Write, err_stream: &mut dyn Write) {
    if let Some(name) = args.get(0) {
        match name.parse::<Command>() {
            Ok(Command::External(path)) => {
                writeln!(iostream, "{} is {}", name, path.to_str().unwrap()).unwrap();
            }
            Ok(
                Command::Exit
                | Command::Echo
                | Command::Type
                | Command::Pwd
                | Command::Cd
                | Command::History,
            ) => {
                writeln!(iostream, "{} is a shell builtin", name).unwrap();
            }
            Ok(Command::Invalid) | Err(_) => {
                writeln!(err_stream, "{}: not found", name).unwrap();
            }
        }
    } else {
        writeln!(err_stream, "type: missing operand").unwrap();
    }
}

pub fn echo_cmd(input: &[String], iostream: &mut dyn Write) {
    writeln!(iostream, "{}", input.join(" ")).unwrap();
}

pub fn exit_cmd(args: &[String]) {
    let code = args.get(0).and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
    process::exit(code);
}
