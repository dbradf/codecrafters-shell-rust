use std::process::Command;

pub fn execute_command(command: &str, args: &[String]) {
    Command::new(command).args(args).status().unwrap();
}
