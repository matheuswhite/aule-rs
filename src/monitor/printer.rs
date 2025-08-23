use crate::monitor::{AsMonitor, Monitor};
use crate::signal::Signal;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use std::println;

pub struct Printer {
    title: String,
    units: Vec<String>,
}

impl Printer {
    pub fn new<const N: usize>(title: &str, units: [&str; N]) -> Self {
        Printer {
            title: title.to_string(),
            units: units.iter().map(|&s| s.to_string()).collect(),
        }
    }
}

impl Monitor for Printer {
    fn show(&mut self, inputs: Vec<Signal>) {
        let values = inputs
            .iter()
            .zip(self.units.iter())
            .map(|(input, unit)| format!("{} {}", input.value, unit))
            .collect::<Vec<_>>()
            .join(", ");
        println!("[{}] {}", self.title, values);
    }
}

impl AsMonitor for Printer {}
