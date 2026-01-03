use std::io::{self, Write};

use crate::builtins::builtin::BuiltinCommand;

pub struct EchoCommand;

impl EchoCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuiltinCommand for EchoCommand {
    fn execute(&self, args: &[String]) {
        println!("{}", args.join(" "));
        io::stdout().flush().unwrap();
    }
}
