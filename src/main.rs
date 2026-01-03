use std::io::{self, Write};

use crate::{
    builtins::register_builtins::register_builtin_commands,
    exec::{execute_command::execute_command, search_path::search_path},
    parse::parse_command,
};

mod builtins;
mod exec;
mod parse;

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
        if let Some(command) = commands.get(input.first().unwrap()) {
            command.execute(&input[1..]);
        } else if search_path(input.first().unwrap()).is_some() {
            execute_command(input.first().unwrap(), &input[1..]);
        } else {
            println!("{}: command not found", &input.first().unwrap());
            io::stdout().flush().unwrap();
        }
    }
}
