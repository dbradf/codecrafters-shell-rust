use std::path::PathBuf;

use crate::builtins::builtin::BuiltinCommand;

pub struct CdCommand;

impl CdCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuiltinCommand for CdCommand {
    fn execute(&self, args: &[&str]) {
        let target_path = args[0];
        if target_path.starts_with('/') {
            let path = PathBuf::from(target_path);
            if path.exists() {
                std::env::set_current_dir(path).unwrap();
            } else {
                println!("cd: {}: No such file or directory", target_path);
            }
        }
    }
}
