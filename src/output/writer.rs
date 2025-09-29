use crate::output::Output;
use crate::signal::Signal;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use std::{
    fs::OpenOptions,
    io::{self, Write},
    time::Duration,
};

pub struct Writter {
    filename: String,
    sim_time: Duration,
}

impl Writter {
    pub fn new<const N: usize>(filename: &str, variable_names: [&str; N]) -> Self {
        let writer = Writter {
            filename: filename.to_string(),
            sim_time: Duration::default(),
        };

        writer
            .write_header(variable_names)
            .expect("Failed to write header");

        writer
    }

    fn write_header<const N: usize>(&self, variable_names: [&str; N]) -> Result<(), io::Error> {
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.filename)?
            .write_all(("t,".to_string() + &variable_names.join(",") + "\n").as_bytes())
    }

    fn append_line(&self, content: String) -> Result<(), io::Error> {
        OpenOptions::new()
            .append(true)
            .open(&self.filename)?
            .write_all(content.as_bytes())
    }
}

impl Output<f32> for Writter {
    fn show(&mut self, inputs: Signal<f32>) {
        self.sim_time += inputs.dt;

        let line = format!("{},{}\n", self.sim_time.as_secs_f32(), inputs.value);
        self.append_line(line).expect("Failed to write data line");
    }
}

impl<const N: usize> Output<[f32; N]> for Writter {
    fn show(&mut self, inputs: Signal<[f32; N]>) {
        self.sim_time += inputs.dt;

        let values: Vec<String> = inputs.value.iter().map(|v| v.to_string()).collect();
        let line = format!("{},{}\n", self.sim_time.as_secs_f32(), values.join(","));
        self.append_line(line).expect("Failed to write data line");
    }
}
