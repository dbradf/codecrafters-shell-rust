use std::{
    io::{BufRead, BufReader, Write},
    process::{Child, Command, Stdio},
};

use crate::tokenize::TokenizedCommand;

pub fn execute_command(command: &TokenizedCommand, stdout: &mut dyn Write, stderr: &mut dyn Write) {
    let child = Command::new(&command.command)
        .args(&command.args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let output = child.wait_with_output().unwrap();
    stdout.write_all(&output.stdout).unwrap();
    stderr.write_all(&output.stderr).unwrap();
}

pub fn execute_pipeline(
    commands: &[TokenizedCommand],
    final_stdout: &mut dyn Write,
    final_stderr: &mut dyn Write,
) {
    let mut prev_child: Option<Child> = None;
    for command in commands {
        let child = if let Some(prev_child) = prev_child {
            Command::new(&command.command)
                .args(&command.args)
                .stdin(Stdio::from(prev_child.stdout.unwrap()))
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap()
        } else {
            Command::new(&command.command)
                .args(&command.args)
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap()
        };

        prev_child = Some(child);
    }

    if let Some(last_child) = prev_child {
        let child_stdout = last_child.stdout.unwrap();
        let stdout_reader = BufReader::new(child_stdout);

        for line in stdout_reader.lines() {
            match line {
                Ok(line) => {
                    final_stdout.write_all(line.as_bytes()).unwrap();
                    final_stdout.write_all("\n".as_bytes()).unwrap();
                    final_stdout.flush().unwrap();
                }
                Err(_) => todo!(),
            }
        }

        let child_stderr = last_child.stderr.unwrap();
        let stderr_reader = BufReader::new(child_stderr);
        for line in stderr_reader.lines() {
            match line {
                Ok(line) => {
                    final_stderr.write_all(line.as_bytes()).unwrap();
                    final_stderr.write_all("\n".as_bytes()).unwrap();
                    final_stderr.flush().unwrap();
                }
                Err(_) => todo!(),
            }
        }
    }
}
