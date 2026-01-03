use std::collections::HashSet;

use crate::builtins::builtin::BuiltinCommand;

pub struct TypeCommand {
    built_ins: HashSet<String>,
}

impl TypeCommand {
    pub fn new(built_ins: HashSet<String>) -> Self {
        Self { built_ins }
    }
}

impl BuiltinCommand for TypeCommand {
    fn execute(&self, args: &[&str]) {
        let command = args.first().unwrap();
        if self.built_ins.contains(*command) {
            println!("{} is a shell builtin", command);
        } else {
            println!("{}: not found", command);
        }
    }
}
