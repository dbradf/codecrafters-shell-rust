use crate::{cmd_output::CmdOutput, tokenize::TokenizedCommand};

pub trait BuiltinCommand {
    fn execute(&self, command: &TokenizedCommand, cmd_output: &mut CmdOutput);
}
