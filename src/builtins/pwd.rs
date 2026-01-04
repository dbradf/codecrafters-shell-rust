use crate::{builtins::builtin::BuiltinCommand, tokenize::TokenizedCommand};

pub struct PwdCommand;

impl PwdCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuiltinCommand for PwdCommand {
    fn execute(&self, _: &TokenizedCommand) {
        let current_dir = std::env::current_dir().unwrap();
        println!("{}", current_dir.canonicalize().unwrap().to_str().unwrap());
    }
}
