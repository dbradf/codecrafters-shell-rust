use std::io::{self, Write};

use crate::builtins::register_builtins::register_builtin_commands;

mod builtins;
mod exec;

fn main() {
    repl();
}

fn repl() {
    let commands = register_builtin_commands();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        let input = parse_command(&buffer);
        if let Some(command) = commands.get(*input.first().unwrap()) {
            command.execute(&input[1..]);
        } else {
            println!("{}: command not found", &input.first().unwrap());
            io::stdout().flush().unwrap();
        }
    }
}

fn parse_command(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}
