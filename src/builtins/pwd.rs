use crate::builtins::builtin::BuiltinCommand;

pub struct PwdCommand;

impl PwdCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuiltinCommand for PwdCommand {
    fn execute(&self, _: &[String]) {
        let current_dir = std::env::current_dir().unwrap();
        println!("{}", current_dir.canonicalize().unwrap().to_str().unwrap());
    }
}
