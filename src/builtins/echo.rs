use crate::{builtins::builtin::BuiltinCommand, cmd_output::CmdOutput, tokenize::TokenizedCommand};

pub struct EchoCommand;

impl EchoCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuiltinCommand for EchoCommand {
    fn execute(&self, command: &TokenizedCommand, cmd_output: &mut CmdOutput) {
        cmd_output.output(&format!("{}\n", &command.args.join(" ")));
    }
}
