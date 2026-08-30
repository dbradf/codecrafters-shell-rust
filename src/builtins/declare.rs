use std::io::Write;

use crate::{builtins::builtin::BuiltinCommand, tokenize::TokenizedCommand};

pub struct DeclareCommand;

impl DeclareCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuiltinCommand for DeclareCommand {
    fn execute(&self, command: &TokenizedCommand, output: &mut dyn Write, error: &mut dyn Write) {
        let args = parse(command);
        if args.print_description {
            output
                .write_fmt(format_args!("declare: {}: not found\n", args.name))
                .unwrap();
            output.flush().unwrap();
        } else {
            dbg!(command);
            todo!();
        }
    }
}

#[derive(Debug)]
struct DeclareArgs {
    print_description: bool,
    name: String,
}

fn parse(command: &TokenizedCommand) -> DeclareArgs {
    let mut parsed = DeclareArgs {
        print_description: false,
        name: "".to_string(),
    };

    for arg in &command.args {
        match arg.as_str() {
            "-p" => parsed.print_description = true,
            _ => parsed.name = arg.clone(),
        }
    }

    parsed
}
