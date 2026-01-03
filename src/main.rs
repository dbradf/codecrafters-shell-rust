use std::{
    collections::{HashMap, HashSet},
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
    let mut known_commands: HashSet<String> = commands.keys().map(|k| k.to_string()).collect();
    // hack: manually add "type", since it is not already in the key list.
    // and new builtins will need to be added before this.
    known_commands.insert(String::from("type"));
    commands.insert(
        String::from("type"),
        Box::new(TypeCommand::new(known_commands)),
    );
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

struct TypeCommand {
    built_ins: HashSet<String>,
}

impl TypeCommand {
    fn new(built_ins: HashSet<String>) -> Self {
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
