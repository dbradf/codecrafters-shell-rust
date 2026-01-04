use std::collections::HashSet;

use crate::{
    builtins::builtin::BuiltinCommand, cmd_output::CmdOutput, exec::search_path::search_path,
    tokenize::TokenizedCommand,
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
    fn execute(&self, command: &TokenizedCommand, cmd_output: &mut CmdOutput) {
        let command = command.args.first().unwrap();
        if self.built_ins.contains(command) {
            cmd_output.output(&format!("{} is a shell builtin\n", command));
        } else if let Some(path) = search_path(command) {
            cmd_output.output(&format!(
                "{} is {}\n",
                command,
                path.canonicalize().unwrap().to_str().unwrap()
            ));
        } else {
            cmd_output.error(&format!("{}: not found\n", command));
        }
    }
}
