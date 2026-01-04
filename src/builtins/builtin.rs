use std::io::Write;

use crate::tokenize::TokenizedCommand;

pub trait BuiltinCommand {
    fn execute(&self, command: &TokenizedCommand, output: &mut dyn Write, error: &mut dyn Write);
}
