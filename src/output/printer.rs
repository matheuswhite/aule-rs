use crate::block::Block;
use crate::signal::Signal;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use std::println;

#[derive(Debug, Clone, PartialEq)]
pub struct Printer<const N: usize> {
    title: String,
    units: [String; N],
}

impl<const N: usize> Printer<N> {
    pub fn new(title: &str, units: [&str; N]) -> Self {
        Printer {
            title: title.to_string(),
            units: units.map(|s| s.to_string()),
        }
    }
}

impl<const N: usize> Block for Printer<N> {
    type Input = [f32; N];
    type Output = [f32; N];

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        let values = input
            .value
            .iter()
            .zip(self.units.iter())
            .map(|(input, unit)| format!("{} {}", input, unit))
            .collect::<Vec<_>>()
            .join(", ");
        println!("[{}] {}", self.title, values);

        input
    }
}
