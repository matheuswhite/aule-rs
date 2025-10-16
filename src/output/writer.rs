use crate::block::Block;
use crate::signal::Signal;
use crate::time::TimeType;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::marker::PhantomData;
use std::{
    fs::OpenOptions,
    io::{self, Write},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Writter<const N: usize, T>
where
    T: TimeType,
{
    filename: String,
    _marker: PhantomData<[T; N]>,
}

impl<const N: usize, T> Writter<N, T>
where
    T: TimeType,
{
    pub fn new(filename: &str, variable_names: [&str; N]) -> Self {
        let writer = Self {
            filename: filename.to_string(),
            _marker: PhantomData,
        };

        writer
            .write_header(variable_names)
            .expect("Failed to write header");

        writer
    }

    fn write_header(&self, variable_names: [&str; N]) -> Result<(), io::Error> {
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

impl<const N: usize, T> Block for Writter<N, T>
where
    T: TimeType,
{
    type Input = [f32; N];
    type Output = [f32; N];
    type TimeType = T;

    fn output(
        &mut self,
        input: Signal<Self::Input, Self::TimeType>,
    ) -> Signal<Self::Output, Self::TimeType> {
        let values: Vec<String> = input.value.iter().map(|v| v.to_string()).collect();
        let line = format!(
            "{},{}\n",
            input.delta.sim_time().as_secs_f32(),
            values.join(",")
        );
        self.append_line(line).expect("Failed to write data line");

        input
    }
}
