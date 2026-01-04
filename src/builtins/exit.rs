use crate::{builtins::builtin::BuiltinCommand, tokenize::TokenizedCommand};

pub struct ExitCommand;

impl ExitCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuiltinCommand for ExitCommand {
    fn execute(&self, _: &TokenizedCommand) {
        std::process::exit(0);
    }
}
