use std::{
    io::{Read, Write},
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
    let mut previous_output: Option<String> = None;
    for command in commands {
        let mut child = Command::new(&command.command)
            .args(&command.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        if let Some(previous) = previous_output {
            child
                .stdin
                .unwrap()
                .write_all(previous.as_bytes())
                .expect("error reading stdin");
        }

        let mut buffer = String::new();
        child.stdout.unwrap().read_to_string(&mut buffer);
        previous_output = Some(buffer);
    }

    if let Some(output) = previous_output {
        final_stdout.write_all(output.as_bytes());
    }
}
