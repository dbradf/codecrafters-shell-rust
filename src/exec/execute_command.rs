use std::process::Command;

use crate::tokenize::TokenizedCommand;

pub fn execute_command(command: &TokenizedCommand) {
    Command::new(&command.command)
        .args(&command.args)
        .status()
        .unwrap();
}
