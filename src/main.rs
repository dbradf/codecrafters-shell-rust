use std::{
    collections::HashMap,
    io::{self, Write},
    process::exit,
};

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

fn register_builtin_commands() -> HashMap<String, Box<dyn BuiltinCommand>> {
    let mut commands: HashMap<String, Box<dyn BuiltinCommand>> = HashMap::new();
    commands.insert(String::from("echo"), Box::new(EchoCommand::new()));
    commands.insert(String::from("exit"), Box::new(ExitCommand::new()));
    commands
}

trait BuiltinCommand {
    fn execute(&self, args: &[&str]);
}

struct EchoCommand;

impl EchoCommand {
    fn new() -> Self {
        Self {}
    }
}

impl BuiltinCommand for EchoCommand {
    fn execute(&self, args: &[&str]) {
        println!("{}", args.join(" "));
        io::stdout().flush().unwrap();
    }
}

struct ExitCommand;

impl ExitCommand {
    fn new() -> Self {
        Self {}
    }
}

impl BuiltinCommand for ExitCommand {
    fn execute(&self, _: &[&str]) {
        exit(0);
    }
}
