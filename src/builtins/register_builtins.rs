use std::collections::{HashMap, HashSet};

use crate::builtins::{
    builtin::BuiltinCommand, echo::EchoCommand, exit::ExitCommand, type_cmd::TypeCommand,
};

pub fn register_builtin_commands() -> HashMap<String, Box<dyn BuiltinCommand>> {
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
