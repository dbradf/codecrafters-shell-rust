use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
};

pub struct CmdOutput {
    stdout: Option<File>,
}

impl CmdOutput {
    pub fn new(stdout: &Option<String>) -> Self {
        let stdout_file = stdout.as_ref().map(|stdout| {
            OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(stdout)
                .unwrap()
        });
        Self {
            stdout: stdout_file,
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

    pub fn error(&self, message: &str) {
        print!("{}", message);
        io::stdout().flush().unwrap();
    }
}
