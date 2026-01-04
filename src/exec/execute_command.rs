use std::{
    fs::OpenOptions,
    io::{self, Write},
    process::Command,
};

use crate::tokenize::TokenizedCommand;

pub fn execute_command(command: &TokenizedCommand) {
    let output = Command::new(&command.command)
        .args(&command.args)
        .output()
        .unwrap();

    if let Some(stdout) = &command.stdout {
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(!command.append_stdout)
            .append(command.append_stdout)
            .write(true)
            .open(stdout)
            .unwrap();
        file.write_all(&output.stdout).unwrap();
    } else {
        io::stdout().write_all(&output.stdout).unwrap();
    };

    if let Some(stderr) = &command.stderr {
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(!command.append_stderr)
            .append(command.append_stderr)
            .write(true)
            .open(stderr)
            .unwrap();
        file.write_all(&output.stderr).unwrap();
    } else {
        io::stdout().write_all(&output.stderr).unwrap();
    };
}
