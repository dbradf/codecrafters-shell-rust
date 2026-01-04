use crate::tokenize::TokenizedCommand;

pub trait BuiltinCommand {
    fn execute(&self, command: &TokenizedCommand);
}
