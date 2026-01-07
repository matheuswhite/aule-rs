use crate::block::Block;
use crate::signal::Signal;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::fmt::Display;
use core::marker::PhantomData;
use std::println;

#[derive(Debug, Clone, PartialEq)]
pub struct Printer<const N: usize, T>
where
    T: Display,
{
    title: String,
    units: [String; N],
    _marker: PhantomData<T>,
}

impl<const N: usize, T> Printer<N, T>
where
    T: Display,
{
    pub fn new(title: &str, units: [&str; N]) -> Self {
        Self {
            title: title.to_string(),
            units: units.map(|s| s.to_string()),
            _marker: PhantomData,
        }
    }
}

impl<const N: usize, T> Block for Printer<N, T>
where
    T: Display,
{
    type Input = [T; N];
    type Output = [T; N];

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
