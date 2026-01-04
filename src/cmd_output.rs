use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
};

pub struct CmdOutput {
    stdout: Option<File>,
    stderr: Option<File>,
}

impl CmdOutput {
    pub fn new(stdout: &Option<String>, stderr: &Option<String>) -> Self {
        let stdout_file = stdout.as_ref().map(|stdout| {
            OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(stdout)
                .unwrap()
        });
        let stderr_file = stderr.as_ref().map(|stderr| {
            OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(stderr)
                .unwrap()
        });
        Self {
            stdout: stdout_file,
            stderr: stderr_file,
        }
    }

    pub fn output(&mut self, message: &str) {
        if let Some(output) = &mut self.stdout {
            let _ = output.write_all(message.as_bytes());
        } else {
            print!("{}", message);
            io::stdout().flush().unwrap();
        }
    }

    pub fn error(&mut self, message: &str) {
        if let Some(stderr) = &mut self.stderr {
            let _ = stderr.write_all(message.as_bytes());
        } else {
            print!("{}", message);
            io::stdout().flush().unwrap();
        }
    }
}
