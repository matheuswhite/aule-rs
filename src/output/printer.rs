use crate::output::Output;
use crate::signal::Signal;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use std::println;

#[derive(Debug, Clone, PartialEq)]
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

impl Output<f32> for Printer {
    fn show(&mut self, inputs: Signal<f32>) {
        let unit = if self.units.len() == 1 {
            self.units[0].as_str()
        } else {
            ""
        };
        println!("[{}] {}", self.title, format!("{} {}", inputs.value, unit));
    }
}
impl<const N: usize> Output<[f32; N]> for Printer {
    fn show(&mut self, inputs: Signal<[f32; N]>) {
        let values = inputs
            .value
            .iter()
            .zip(self.units.iter())
            .map(|(input, unit)| format!("{} {}", input, unit))
            .collect::<Vec<_>>()
            .join(", ");
        println!("[{}] {}", self.title, values);
    }
}
