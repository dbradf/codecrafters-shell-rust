use crate::builtins::builtin::BuiltinCommand;

pub struct ExitCommand;

impl ExitCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuiltinCommand for ExitCommand {
    fn execute(&self, _: &[String]) {
        std::process::exit(0);
    }
}
