use std::path::PathBuf;

use crate::{builtins::builtin::BuiltinCommand, cmd_output::CmdOutput, tokenize::TokenizedCommand};

pub struct CdCommand;

impl CdCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuiltinCommand for CdCommand {
    fn execute(&self, command: &TokenizedCommand, cmd_output: &mut CmdOutput) {
        let target_path = &command.args[0];
        let path = if target_path.starts_with(std::path::MAIN_SEPARATOR) {
            PathBuf::from(target_path)
        } else {
            let directory_stack: Vec<&str> = target_path.split(std::path::MAIN_SEPARATOR).collect();
            let cwd = std::env::current_dir().unwrap();
            traverse_directories(&directory_stack, cwd)
        };

        if path.exists() {
            std::env::set_current_dir(path).unwrap();
        } else {
            cmd_output.error(&format!("cd: {}: No such file or directory\n", target_path));
        }
    }
}

fn traverse_directories(directory_stack: &[&str], cwd: PathBuf) -> PathBuf {
    if let Some(next_dir) = directory_stack.first() {
        match *next_dir {
            "~" => {
                let home = std::env::var("HOME").unwrap();
                return traverse_directories(&directory_stack[1..], PathBuf::from(home));
            }
            "." => {
                return traverse_directories(&directory_stack[1..], cwd);
            }
            ".." => {
                return traverse_directories(
                    &directory_stack[1..],
                    cwd.parent().unwrap().to_path_buf(),
                );
            }
            _ => return traverse_directories(&directory_stack[1..], cwd.join(*next_dir)),
        }
    }

    cwd
}
