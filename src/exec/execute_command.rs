use std::{io::Write, process::Command};

use crate::tokenize::TokenizedCommand;

pub fn execute_command(command: &TokenizedCommand, stdout: &mut dyn Write, stderr: &mut dyn Write) {
    let output = Command::new(&command.command)
        .args(&command.args)
        .output()
        .unwrap();

    stdout.write_all(&output.stdout).unwrap();
    stderr.write_all(&output.stderr).unwrap();
}
