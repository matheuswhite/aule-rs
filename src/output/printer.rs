use crate::block::Block;
use crate::signal::Signal;
use crate::time::TimeType;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::fmt::Display;
use core::marker::PhantomData;
use std::println;

#[derive(Debug, Clone, PartialEq)]
pub struct Printer<const N: usize, T, K>
where
    T: Display,
    K: TimeType,
{
    title: String,
    units: [String; N],
    _marker: PhantomData<(T, K)>,
}

impl<const N: usize, T, K> Printer<N, T, K>
where
    T: Display,
    K: TimeType,
{
    pub fn new(title: &str, units: [&str; N]) -> Self {
        Self {
            title: title.to_string(),
            units: units.map(|s| s.to_string()),
            _marker: PhantomData,
        }
    }
}

impl<const N: usize, T, K> Block for Printer<N, T, K>
where
    T: Display,
    K: TimeType,
{
    type Input = [T; N];
    type Output = [T; N];
    type TimeType = K;

    fn output(
        &mut self,
        input: Signal<Self::Input, Self::TimeType>,
    ) -> Signal<Self::Output, Self::TimeType> {
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
