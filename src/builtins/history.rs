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
    fn execute(&self, command: &TokenizedCommand, output: &mut dyn Write, _error: &mut dyn Write) {
        let limit: Option<usize> = if !command.args.is_empty() {
            Some(command.args[0].parse().unwrap())
        } else {
            None
        };
        let path = PathBuf::from(env::var("HOME").unwrap());
        let history = fs::read_to_string(path.join(HISTORY_FILE)).unwrap_or_default();
        let history_lines: Vec<String> = history
            .lines()
            .enumerate()
            .map(|(index, line)| format!("    {}  {}", index + 1, line))
            .collect();

        let start = if let Some(limit) = limit {
            history_lines.len() - limit
        } else {
            0
        };

        for line in history_lines[start..].iter() {
            output.write_fmt(format_args!("{}\n", line)).unwrap();
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

pub fn reset_history() {
    let path = PathBuf::from(env::var("HOME").unwrap());
    let history_path = path.join(HISTORY_FILE);
    if history_path.exists() {
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(history_path)
            .unwrap();
    }
}
