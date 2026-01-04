use crate::{builtins::builtin::BuiltinCommand, cmd_output::CmdOutput, tokenize::TokenizedCommand};

pub struct PwdCommand;

impl PwdCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuiltinCommand for PwdCommand {
    fn execute(&self, _: &TokenizedCommand, cmd_output: &mut CmdOutput) {
        let current_dir = std::env::current_dir().unwrap();
        let canonicalized_path = current_dir.canonicalize().unwrap();
        let path = canonicalized_path.to_str().unwrap();
        cmd_output.output(&format!("{}\n", path));
    }
}
