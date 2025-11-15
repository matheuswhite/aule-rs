use crate::block::Block;
use crate::signal::Signal;
use crate::time::TimeType;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::marker::PhantomData;
use std::fs;
use std::path::Path;
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
    variable_names: [String; N],
    _marker: PhantomData<T>,
}

impl<const N: usize, T> Writter<N, T>
where
    T: TimeType,
{
    pub fn new(filename: &str, variable_names: [&str; N]) -> Self {
        let writer = Self {
            filename: filename.to_string(),
            variable_names: variable_names.map(|s| s.to_string()),
            _marker: PhantomData,
        };

        writer
            .write_header(&variable_names)
            .expect("Failed to write header");

        writer
    }

    fn write_header(&self, variable_names: &[&str]) -> Result<(), io::Error> {
        fs::create_dir_all(Path::new(&self.filename).parent().unwrap_or(Path::new(""))).ok();

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

    fn reset(&mut self) {
        std::fs::remove_file(&self.filename).ok();

        let variable_names = self
            .variable_names
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>();
        self.write_header(&variable_names)
            .expect("Failed to reset writer");
    }
}
