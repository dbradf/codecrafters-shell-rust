use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::builtins::{
    builtin::BuiltinCommand, cd::CdCommand, declare::DeclareCommand, echo::EchoCommand,
    exit::ExitCommand, history::HistoryCommand, pwd::PwdCommand, type_cmd::TypeCommand,
};

pub fn register_builtin_commands(
    history: Rc<HistoryCommand>,
) -> HashMap<String, Rc<dyn BuiltinCommand>> {
    let mut commands: HashMap<String, Rc<dyn BuiltinCommand>> = HashMap::new();
    commands.insert(String::from("cd"), Rc::new(CdCommand::new()));
    commands.insert(String::from("declare"), Rc::new(DeclareCommand::new()));
    commands.insert(String::from("echo"), Rc::new(EchoCommand::new()));
    commands.insert(
        String::from("exit"),
        Rc::new(ExitCommand::new(history.clone())),
    );
    commands.insert(String::from("history"), history);
    commands.insert(String::from("pwd"), Rc::new(PwdCommand::new()));
    let mut known_commands: HashSet<String> = commands.keys().map(|k| k.to_string()).collect();
    // hack: manually add "type", since it is not already in the key list.
    // and new builtins will need to be added before this.
    known_commands.insert(String::from("type"));
    commands.insert(
        String::from("type"),
        Rc::new(TypeCommand::new(known_commands)),
    );
    commands
}
