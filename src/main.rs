use std::io::{self, Write};

use crate::{
    builtins::register_builtins::register_builtin_commands,
    exec::{execute_command::execute_command, search_path::search_path},
    tokenize::tokenize_input,
};

mod builtins;
mod exec;
mod tokenize;

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

        let input = tokenize_input(&buffer);
        if let Some(command) = commands.get(&input.command) {
            command.execute(&input);
        } else if search_path(&input.command).is_some() {
            execute_command(&input);
        } else {
            println!("{}: command not found", &input.command);
            io::stdout().flush().unwrap();
        }
    }
}
