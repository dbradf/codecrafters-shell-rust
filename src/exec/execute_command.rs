use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
    process::{ChildStderr, ChildStdout, Command, Stdio},
};

use os_pipe::PipeReader;

use crate::{builtins::builtin::BuiltinCommand, tokenize::TokenizedCommand};

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

struct PrevOutput {
    stdout: ReaderSource,
    stderr: ReaderSource,
}

pub fn execute_pipeline(
    commands: &[TokenizedCommand],
    builtins: &HashMap<String, Box<dyn BuiltinCommand>>,
    final_stdout: &mut dyn Write,
    final_stderr: &mut dyn Write,
) {
    // let mut prev_child: Option<Child> = None;
    let mut prev_child: Option<PrevOutput> = None;
    for command in commands {
        if let Some(builtin) = builtins.get(&command.command) {
            let (output_reader, mut output_writer) = os_pipe::pipe().unwrap();
            let (error_reader, mut error_writer) = os_pipe::pipe().unwrap();
            builtin.execute(command, &mut output_writer, &mut error_writer);
            prev_child = Some(PrevOutput {
                stdout: ReaderSource::Pipe(output_reader),
                stderr: ReaderSource::Pipe(error_reader),
            });
        }
        let child = if let Some(prev_child) = prev_child {
            Command::new(&command.command)
                .args(&command.args)
                .stdin(Stdio::from(prev_child.stdout))
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

        prev_child = Some(PrevOutput {
            stdout: ReaderSource::Stdout(child.stdout.unwrap()),
            stderr: ReaderSource::Stderr(child.stderr.unwrap()),
        });
    }

    if let Some(last_child) = prev_child {
        let stdout_reader = BufReader::new(last_child.stdout);

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

        let stderr_reader = BufReader::new(last_child.stderr);
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

enum ReaderSource {
    Pipe(PipeReader),
    Stdout(ChildStdout),
    Stderr(ChildStderr),
}

impl From<ReaderSource> for Stdio {
    fn from(source: ReaderSource) -> Self {
        match source {
            ReaderSource::Pipe(pipe_reader) => Stdio::from(pipe_reader),
            ReaderSource::Stdout(child_stdout) => Stdio::from(child_stdout),
            ReaderSource::Stderr(child_stderr) => Stdio::from(child_stderr),
        }
    }
}

impl Read for ReaderSource {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            ReaderSource::Pipe(pipe_reader) => pipe_reader.read(buf),
            ReaderSource::Stdout(child_stdout) => child_stdout.read(buf),
            ReaderSource::Stderr(child_stderr) => child_stderr.read(buf),
        }
    }
}
