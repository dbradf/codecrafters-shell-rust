use std::{env, io::Write, path::PathBuf, rc::Rc};

use crate::{
    builtins::{builtin::BuiltinCommand, history::HistoryCommand},
    tokenize::TokenizedCommand,
};

pub struct ExitCommand {
    history: Rc<HistoryCommand>,
}

impl ExitCommand {
    pub fn new(history: Rc<HistoryCommand>) -> Self {
        Self { history }
    }
}

impl BuiltinCommand for ExitCommand {
    fn execute(&self, _: &TokenizedCommand, _output: &mut dyn Write, _error: &mut dyn Write) {
        if let Ok(hist_file) = env::var("HISTFILE") {
            self.history.write_history_to_file(&PathBuf::from(hist_file));
        }
        std::process::exit(0);
    }
}
