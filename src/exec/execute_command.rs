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
            .truncate(true)
            .write(true)
            .open(stdout)
            .unwrap();
        file.write_all(&output.stdout).unwrap();
    } else {
        io::stdout().write_all(&output.stdout).unwrap();
    };

    io::stdout().write_all(&output.stderr).unwrap();
}
