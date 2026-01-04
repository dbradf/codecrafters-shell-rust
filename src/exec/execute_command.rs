use std::{
    io::{BufRead, BufReader, Read, Write},
    process::{Child, Command, Stdio},
};

use crate::tokenize::TokenizedCommand;

pub fn execute_command(command: &TokenizedCommand, stdout: &mut dyn Write, stderr: &mut dyn Write) {
    let child = Command::new(&command.command)
        .args(&command.args)
        .spawn()
        .unwrap();

    let output = child.wait_with_output().unwrap();
    stdout.write_all(&output.stdout).unwrap();
    stderr.write_all(&output.stderr).unwrap();
}

pub fn execute_pipeline(
    commands: &[TokenizedCommand],
    final_stdout: &mut dyn Write,
    stderr: &mut dyn Write,
) {
    let mut prev_child: Option<Child> = None;
    for command in commands {
        let child = if let Some(prev_child) = prev_child {
            Command::new(&command.command)
                .args(&command.args)
                .stdin(Stdio::from(prev_child.stdout.unwrap()))
                .stdout(Stdio::piped())
                .spawn()
                .unwrap()
        } else {
            Command::new(&command.command)
                .args(&command.args)
                .stdout(Stdio::piped())
                .spawn()
                .unwrap()
        };

        prev_child = Some(child);
    }

    if let Some(last_child) = prev_child {
        let stdout = last_child.stdout.unwrap();
        let stdout_reader = BufReader::new(stdout);

        for line in stdout_reader.lines() {
            match line {
                Ok(line) => {
                    final_stdout.write_all(line.as_bytes());
                }
                Err(_) => todo!(),
            }
        }
    }
}
