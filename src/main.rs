use std::io::{self, Write};

fn main() {
    repl();
}

fn repl() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        let command = parse_command(&buffer);
        match command.first() {
            Some(&"echo") => {
                println!("{}", &command[1..].join(" "));
                io::stdout().flush().unwrap();
            }
            Some(&"exit") => break,
            _ => {
                println!("{}: command not found", &command.first().unwrap());
                io::stdout().flush().unwrap();
            }
        }
    }
}

fn parse_command(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}
