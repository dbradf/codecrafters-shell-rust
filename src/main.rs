use rustyline::{
    Completer, CompletionType, Config, Editor, Helper, Highlighter, Hinter, Validator,
    history::FileHistory,
};

use crate::{
    builtins::register_builtins::register_builtin_commands,
    cmd_output::CmdOutput,
    completion::TermCompleter,
    exec::{
        execute_command::{execute_command, execute_pipeline},
        find_executables::find_executables_in_path,
        search_path::search_path,
    },
    tokenize::{TokenizeResult, tokenize_input},
};

mod builtins;
mod cmd_output;
mod completion;
mod exec;
mod tokenize;

fn main() {
    repl();
}

fn repl() {
    let commands = register_builtin_commands();
    let external_commands = find_executables_in_path();
    let mut command_names: Vec<String> = commands.keys().map(|c| c.to_string()).collect();
    command_names.extend(external_commands);
    let mut rl = init_readline(&command_names);

    loop {
        let readline = rl.readline("$ ");
        match readline {
            Ok(line) => {
                let input = tokenize_input(&line);
                match input {
                    TokenizeResult::SingleCommand(input) => {
                        let mut cmd_output = CmdOutput::new(
                            &input.stdout,
                            &input.append_stdout,
                            &input.stderr,
                            &input.append_stderr,
                        );
                        if let Some(command) = commands.get(&input.command) {
                            command.execute(&input, &mut cmd_output.stdout, &mut cmd_output.stderr);
                        } else if search_path(&input.command).is_some() {
                            execute_command(&input, &mut cmd_output.stdout, &mut cmd_output.stderr);
                        } else {
                            println!("{}: command not found", &input.command);
                        }
                    }
                    TokenizeResult::Pipeline(tokenized_commands) => {
                        let last_command = tokenized_commands.last().unwrap();
                        let mut cmd_output = CmdOutput::new(
                            &last_command.stdout,
                            &last_command.append_stdout,
                            &last_command.stderr,
                            &last_command.append_stderr,
                        );
                        execute_pipeline(
                            &tokenized_commands,
                            &commands,
                            &mut cmd_output.stdout,
                            &mut cmd_output.stderr,
                        );
                    }
                }
            }
            Err(err) => {
                dbg!(err);
            }
        }
    }
}

#[derive(Completer, Helper, Hinter, Highlighter, Validator)]
struct TermHelper {
    #[rustyline(Completer)]
    completer: TermCompleter,
}

impl TermHelper {
    pub fn new(commands: &[String]) -> Self {
        Self {
            completer: TermCompleter::new(commands),
        }
    }
}

fn init_readline(commands: &[String]) -> Editor<TermHelper, FileHistory> {
    let config = Config::builder()
        .completion_type(CompletionType::List)
        .build();
    let mut rl = Editor::with_config(config).unwrap();
    rl.set_helper(Some(TermHelper::new(commands)));

    rl
}
