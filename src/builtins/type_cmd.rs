use std::{collections::HashSet, io::Write};

use crate::{
    builtins::builtin::BuiltinCommand, exec::search_path::search_path, tokenize::TokenizedCommand,
};

pub struct TypeCommand {
    built_ins: HashSet<String>,
}

impl TypeCommand {
    pub fn new(built_ins: HashSet<String>) -> Self {
        Self { built_ins }
    }
}

impl BuiltinCommand for TypeCommand {
    fn execute(&self, command: &TokenizedCommand, output: &mut dyn Write, error: &mut dyn Write) {
        let command = command.args.first().unwrap();
        if self.built_ins.contains(command) {
            output.write_fmt(format_args!("{} is a shell builtin\n", command));
        } else if let Some(path) = search_path(command) {
            output.write_fmt(format_args!(
                "{} is {}\n",
                command,
                path.canonicalize().unwrap().to_str().unwrap()
            ));
        } else {
            error.write_fmt(format_args!("{}: not found\n", command));
        }
    }
}
