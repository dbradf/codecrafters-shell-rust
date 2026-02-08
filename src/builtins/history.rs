use std::io::Write;

use crate::{builtins::builtin::BuiltinCommand, tokenize::TokenizedCommand};

pub struct HistoryCommand;

impl HistoryCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuiltinCommand for HistoryCommand {
    fn execute(&self, command: &TokenizedCommand, output: &mut dyn Write, _error: &mut dyn Write) {
        let _ = output.write_fmt(format_args!("{}\n", &command.args.join(" ")));
    }
}
