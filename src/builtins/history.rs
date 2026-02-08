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
        let args = HistoryArgs::parse(&command.args);
        if let Some(read_file) = args.read {
            let contents = fs::read_to_string(read_file).unwrap();
            for line in contents.lines() {
                save_command(line);
            }

            return;
        }

        let path = PathBuf::from(env::var("HOME").unwrap());
        let history = fs::read_to_string(path.join(HISTORY_FILE)).unwrap_or_default();
        if let Some(write_file) = args.write {
            let path = PathBuf::from(&write_file);
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .unwrap();

            file.write_all(history.as_bytes()).unwrap();
            return;
        }
        let history_lines: Vec<String> = history
            .lines()
            .enumerate()
            .map(|(index, line)| format!("    {}  {}", index + 1, line))
            .collect();

        let start = if let Some(limit) = args.limit {
            history_lines.len() - limit
        } else {
            0
        };

        for line in history_lines[start..].iter() {
            output.write_fmt(format_args!("{}\n", line)).unwrap();
        }
    }
}

struct HistoryArgs {
    limit: Option<usize>,
    read: Option<String>,
    write: Option<String>,
}

enum ArgsState {
    Default,
    ReadHistory,
    WriteHistory,
}

impl HistoryArgs {
    pub fn parse(args: &[String]) -> Self {
        let mut state = ArgsState::Default;
        let mut read_history = None;
        let mut write_history = None;
        let mut limit = None;
        for arg in args {
            match state {
                ArgsState::Default => match arg.as_str() {
                    "-r" => {
                        state = ArgsState::ReadHistory;
                    }
                    "-w" => {
                        state = ArgsState::WriteHistory;
                    }
                    _ => {
                        if let Ok(value) = arg.parse() {
                            limit = Some(value);
                        }
                    }
                },
                ArgsState::ReadHistory => {
                    read_history = Some(arg.clone());
                }
                ArgsState::WriteHistory => {
                    write_history = Some(arg.clone());
                }
            }
        }

        Self {
            limit,
            read: read_history,
            write: write_history,
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
        let _ = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(history_path)
            .unwrap();
    }
}
