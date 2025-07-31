use std::{
    fs::OpenOptions,
    io::{self, Write},
    time::Duration,
};

use crate::block::{AsMonitor, Monitor, Signal};

pub struct Writter {
    filename: String,
    sim_time: Duration,
}

impl Writter {
    pub fn new(filename: &str, variable_name: &str) -> Self {
        let writer = Writter {
            filename: filename.to_string(),
            sim_time: Duration::default(),
        };

        writer
            .write_header(variable_name)
            .expect("Failed to write header");

        writer
    }

    fn write_header(&self, variable_name: &str) -> Result<(), io::Error> {
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.filename)?
            .write_all(format!("t;{}\n", variable_name).as_bytes())
    }

    fn append_line(&self, content: String) -> Result<(), io::Error> {
        OpenOptions::new()
            .append(true)
            .open(&self.filename)?
            .write_all(content.as_bytes())
    }
}

impl Monitor for Writter {
    fn show(&mut self, inputs: Signal) {
        self.sim_time += inputs.dt;

        let line =
            format!("{:.02};{:.02}\n", self.sim_time.as_secs_f32(), inputs.value).replace(".", ",");
        self.append_line(line).expect("Failed to write data line");
    }
}

impl AsMonitor for Writter {}
