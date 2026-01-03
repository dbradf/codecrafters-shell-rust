use std::io::{self, Write};

fn main() {
    repl();
}

fn repl() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        if command.trim_end() == "exit" {
            break;
        }

        println!("{}: command not found", &command.trim_end());
        io::stdout().flush().unwrap();
    }
}
