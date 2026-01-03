use std::{collections::HashSet, env, path::PathBuf};

use is_executable::IsExecutable;

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
        } else if let Some(path) = search_path(command) {
            println!(
                "{} is {}",
                command,
                path.canonicalize().unwrap().to_str().unwrap()
            )
        } else {
            println!("{}: not found", command);
        }
    }
}

fn search_path(command: &str) -> Option<PathBuf> {
    if let Ok(paths_to_search) = env::var("PATH") {
        for path in env::split_paths(&paths_to_search) {
            let maybe_path = path.join(command);
            if maybe_path.exists() && maybe_path.is_executable() {
                return Some(maybe_path);
            }
        }
    }

    None
}
