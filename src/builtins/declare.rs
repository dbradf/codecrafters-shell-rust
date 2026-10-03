use anyhow::Result;
use std::{cell::RefCell, collections::HashMap, io::Write};

use crate::{builtins::builtin::BuiltinCommand, tokenize::TokenizedCommand};

pub struct DeclareCommand {
    variables: RefCell<HashMap<String, String>>,
}

impl DeclareCommand {
    pub fn new() -> Self {
        Self {
            variables: RefCell::new(HashMap::new()),
        }
    }
}

impl BuiltinCommand for DeclareCommand {
    fn execute(&self, command: &TokenizedCommand, output: &mut dyn Write, error: &mut dyn Write) {
        let args = parse(command);
        if let Some(value) = args.value {
            if !is_name_valid(&args.name) {
                output
                    .write_fmt(format_args!(
                        "declare: `{}={}': not a valid identifier\n",
                        args.name, value
                    ))
                    .unwrap()
            } else {
                self.variables.borrow_mut().insert(args.name, value);
            }
        } else if args.print_description {
            if let Some(value) = self.variables.borrow().get(&args.name) {
                output
                    .write_fmt(format_args!("declare -- {}=\"{}\"\n", args.name, value))
                    .unwrap();
            } else {
                output
                    .write_fmt(format_args!("declare: {}: not found\n", args.name))
                    .unwrap();
            }
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
    value: Option<String>,
}

fn parse(command: &TokenizedCommand) -> DeclareArgs {
    let mut parsed = DeclareArgs {
        print_description: false,
        name: "".to_string(),
        value: None,
    };

    for arg in &command.args {
        match arg.as_str() {
            "-p" => parsed.print_description = true,
            var => {
                if var.contains("=") {
                    let parts: Vec<&str> = var.split("=").collect();
                    parsed.name = parts[0].to_string();
                    parsed.value = Some(parts[1].to_string());
                } else {
                    parsed.name = arg.clone();
                }
            }
        }
    }

    parsed
}

fn is_name_valid(name: &str) -> bool {
    if name.len() <= 0 {
        return false;
    }

    let first = name.chars().nth(0);
    if !(first == Some('_') || first.unwrap_or_default().is_alphabetic()) {
        return false;
    }

    name.chars()
        .skip(1)
        .all(|ch| ch.is_alphanumeric() || ch == '_')
}
