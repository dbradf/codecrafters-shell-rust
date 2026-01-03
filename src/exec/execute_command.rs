use std::process::Command;

pub fn execute_command(command: &str, args: &[&str]) {
    Command::new(command).args(args).status().unwrap();
}
