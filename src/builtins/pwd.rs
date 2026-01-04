use std::io::Write;

use crate::{builtins::builtin::BuiltinCommand, tokenize::TokenizedCommand};

pub struct PwdCommand;

impl PwdCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuiltinCommand for PwdCommand {
    fn execute(&self, _: &TokenizedCommand, output: &mut dyn Write, _error: &mut dyn Write) {
        let current_dir = std::env::current_dir().unwrap();
        let canonicalized_path = current_dir.canonicalize().unwrap();
        let path = canonicalized_path.to_str().unwrap();
        output.write_fmt(format_args!("{}\n", path));
    }
}
