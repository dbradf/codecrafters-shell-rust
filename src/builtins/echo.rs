use std::io::{self, Write};

use crate::{builtins::builtin::BuiltinCommand, tokenize::TokenizedCommand};

pub struct EchoCommand;

impl EchoCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuiltinCommand for EchoCommand {
    fn execute(&self, command: &TokenizedCommand) {
        println!("{}", command.args.join(" "));
        io::stdout().flush().unwrap();
    }
}
