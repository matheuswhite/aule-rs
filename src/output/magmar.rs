use alloc::vec;
use std::{
    format,
    io::{BufRead, BufReader, Write},
    process::{Child, Command, ExitStatus, Stdio},
    string::{String, ToString},
    sync::mpsc::{Receiver, channel},
    thread::{self},
    vec::Vec,
};

#[derive(Debug)]
pub struct Magmar {
    child: Child,
    result_channel: Receiver<Result<String, String>>,
}

impl Magmar {
    pub fn new(title: impl AsRef<str>, is_light: bool) -> Self {
        let args = if is_light {
            vec!["-t", title.as_ref(), "--light"]
        } else {
            vec!["-t", title.as_ref()]
        };
        let mut child = Command::new("magmar")
            .args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start Magmar process. Please ensure magmar is installed and in your PATH or install it using `cargo install magmar`.");

        let (tx_channel, result_channel) = channel();

        let stdout = child.stdout.take().unwrap();
        let tx_stdout = tx_channel.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        tx_stdout.send(Ok(l)).ok();
                    }
                    Err(e) => {
                        tx_stdout.send(Err(e.to_string())).ok();
                    }
                }
            }
        });

        let stderr = child.stderr.take().unwrap();
        let tx_stderr = tx_channel.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                match line {
                    Ok(l) if !l.is_empty() => {
                        tx_stderr.send(Err(l)).ok();
                    }
                    Err(e) => {
                        tx_stderr.send(Err(e.to_string())).ok();
                    }
                    _ => {}
                }
            }
        });

        Self {
            child,
            result_channel,
        }
    }

    pub fn send_labels(&mut self, labels: impl AsRef<str>) {
        if let Some(stdin) = self.child.stdin.as_mut() {
            stdin
                .write_all(format!("{}\n", labels.as_ref()).as_bytes())
                .expect("Failed to write labels to Magmar stdin");
        }
    }

    pub fn send_data(&mut self, data: &[f64]) {
        if let Some(stdin) = self.child.stdin.as_mut() {
            let data_str = data
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",");
            stdin
                .write_all(format!("{}\n", data_str).as_bytes())
                .expect("Failed to write to Magmar stdin");
        }
    }

    pub fn send_command(
        &mut self,
        command: impl AsRef<str>,
        result_stdout: impl AsRef<str>,
    ) -> Result<String, String> {
        if let Some(stdin) = self.child.stdin.as_mut() {
            stdin
                .write_all(format!("{}\n", command.as_ref()).as_bytes())
                .expect("Failed to write command to Magmar stdin");

            while let Ok(result) = self.result_channel.recv() {
                match result {
                    Ok(output) => {
                        if output.contains(result_stdout.as_ref()) {
                            return Ok(output);
                        }
                    }
                    Err(err) => return Err(err),
                }
            }
        }

        Err("Magmar closes unexpectedly".to_string())
    }

    pub fn wait(&mut self) -> std::io::Result<ExitStatus> {
        self.child.wait()
    }

    pub fn kill(&mut self) -> std::io::Result<()> {
        self.child.kill()
    }
}
