use std::{
    fs::OpenOptions,
    io::{self, Write},
};

pub struct CmdOutput {
    pub stdout: Box<dyn Write>,
    pub stderr: Box<dyn Write>,
}

impl CmdOutput {
    pub fn new(
        stdout: &Option<String>,
        append_stdout: &bool,
        stderr: &Option<String>,
        append_stderr: &bool,
    ) -> Self {
        let stdout_writer: Box<dyn Write> = if let Some(stdout_file) = stdout {
            Box::new(
                OpenOptions::new()
                    .create(true)
                    .truncate(!*append_stdout)
                    .append(*append_stdout)
                    .write(true)
                    .open(stdout_file)
                    .unwrap(),
            )
        } else {
            Box::new(io::stdout())
        };
        let stderr_writer: Box<dyn Write> = if let Some(stderr_file) = stderr {
            Box::new(
                OpenOptions::new()
                    .create(true)
                    .truncate(!*append_stderr)
                    .append(*append_stderr)
                    .write(true)
                    .open(stderr_file)
                    .unwrap(),
            )
        } else {
            Box::new(io::stderr())
        };

        Self {
            stdout: stdout_writer,
            stderr: stderr_writer,
        }
    }
}
