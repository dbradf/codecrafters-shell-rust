use std::io::Write;

use crate::{builtins::builtin::BuiltinCommand, tokenize::TokenizedCommand};

pub struct DeclareCommand;

impl DeclareCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuiltinCommand for DeclareCommand {
    fn execute(&self, command: &TokenizedCommand, output: &mut dyn Write, _error: &mut dyn Write) {
        todo!();
    }
}
