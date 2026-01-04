use crate::{builtins::builtin::BuiltinCommand, cmd_output::CmdOutput, tokenize::TokenizedCommand};

pub struct ExitCommand;

impl ExitCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuiltinCommand for ExitCommand {
    fn execute(&self, _: &TokenizedCommand, _: &mut CmdOutput) {
        std::process::exit(0);
    }
}
