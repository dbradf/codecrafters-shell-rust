use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

use crate::{builtins::builtin::BuiltinCommand, tokenize::TokenizedCommand};

const HISTORY_FILE: &str = ".cc-sh-history";

pub struct HistoryCommand;

impl HistoryCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuiltinCommand for HistoryCommand {
    fn execute(&self, _command: &TokenizedCommand, output: &mut dyn Write, _error: &mut dyn Write) {
        let path = PathBuf::from(env::var("HOME").unwrap());
        let history = fs::read_to_string(path.join(HISTORY_FILE)).unwrap_or_default();
        for (index, line) in history.lines().enumerate() {
            output
                .write_fmt(format_args!("    {}  {}\n", index, line))
                .unwrap();
        }
    }
}

pub fn save_command(command: &str) {
    let path = PathBuf::from(env::var("HOME").unwrap());
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path.join(HISTORY_FILE))
        .unwrap();

    file.write_all(format!("{}\n", command).as_bytes()).unwrap();
}
