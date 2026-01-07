use crate::block::Block;
use crate::signal::Signal;
use alloc::vec::Vec;
use core::marker::PhantomData;
use num_traits::real::Real;
use std::boxed::Box;
use std::format;
use std::io::{Read, Write};
use std::process::{Child, Command, Stdio};
use std::string::String;
use std::string::ToString;

#[derive(Debug)]
pub struct Plotter<const N: usize, T>
where
    T: Real + ToString,
{
    data: Vec<[Signal<T>; N]>,
    child: Option<Child>,
    title: String,
}

#[derive(Debug)]
pub struct RTPlotter<const N: usize, T>
where
    T: Real + ToString,
{
    child: Option<Child>,
    title: String,
    _marker: PhantomData<[T; N]>,
}

pub trait Joinable {
    fn join(&mut self);
}

pub trait Savable {
    fn save(&mut self, path: &str) -> Result<String, String>;
}

impl<const N: usize, T> Plotter<N, T>
where
    T: Real + ToString,
{
    pub fn new(title: String) -> Self {
        Self {
            data: Vec::new(),
            child: None,
            title,
        }
    }

    pub fn display(&mut self) {
        self.child = Some(
            Command::new("iris")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .arg("-t")
                .arg(&self.title)
                .spawn()
                .unwrap(),
        );

        for signals in &self.data {
            let time = &signals[0].delta.sim_time().as_secs_f32();

            if let Some(child) = &self.child {
                child
                    .stdin
                    .as_ref()
                    .unwrap()
                    .write_all(
                        format!(
                            "{},{}\n",
                            time,
                            signals
                                .iter()
                                .map(|s| s.value.to_string())
                                .collect::<Vec<_>>()
                                .join(",")
                        )
                        .as_bytes(),
                    )
                    .unwrap();
            }
        }
    }
}

impl<const N: usize, T> RTPlotter<N, T>
where
    T: Real + ToString,
{
    pub fn new(title: String) -> Self {
        Self {
            child: None,
            title,
            _marker: PhantomData,
        }
    }
}

impl<const N: usize, T> Block for Plotter<N, T>
where
    T: Real + ToString,
{
    type Input = [T; N];
    type Output = [T; N];

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        self.data.push(input.value.map(|s| Signal {
            value: s,
            delta: input.delta,
        }));
        input
    }

    fn reset(&mut self) {
        self.data.clear();
        if let Some(child) = &mut self.child {
            child.kill().ok();
            self.child = None;
        }
    }
}

impl<const N: usize, T> Block for RTPlotter<N, T>
where
    T: Real + ToString,
{
    type Input = [T; N];
    type Output = [T; N];

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        if self.child.is_none() {
            let command = Command::new("iris")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .arg("-t")
                .arg(&self.title)
                .spawn()
                .expect("Failed to start iris process");
            self.child = Some(command);
        }

        self.child
            .as_ref()
            .unwrap()
            .stdin
            .as_ref()
            .unwrap()
            .write_all(
                format!(
                    "{},{}\n",
                    input.delta.sim_time().as_secs_f32(),
                    input
                        .value
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                )
                .as_bytes(),
            )
            .unwrap();

        input
    }

    fn reset(&mut self) {
        if let Some(child) = &mut self.child {
            child.kill().ok();
            self.child = None;
        }
    }
}

impl<const N: usize, T> Drop for Plotter<N, T>
where
    T: Real + ToString,
{
    fn drop(&mut self) {
        if let Some(child) = &mut self.child {
            child.kill().unwrap();
        }
    }
}

impl<const N: usize, T> Drop for RTPlotter<N, T>
where
    T: Real + ToString,
{
    fn drop(&mut self) {
        if let Some(child) = &mut self.child {
            child.kill().unwrap();
        }
    }
}

impl<const N: usize, T> Joinable for Plotter<N, T>
where
    T: Real + ToString,
{
    fn join(&mut self) {
        if let Some(child) = &mut self.child {
            child.wait().ok();
        }
    }
}

impl<const N: usize, T> Joinable for RTPlotter<N, T>
where
    T: Real + ToString,
{
    fn join(&mut self) {
        if let Some(child) = &mut self.child {
            child.wait().ok();
        }
    }
}

impl<const N: usize, T> Savable for Plotter<N, T>
where
    T: Real + ToString,
{
    fn save(&mut self, path: &str) -> Result<String, String> {
        let Some(child) = self.child.as_mut() else {
            return Err("Plotter process is not running.".to_string());
        };

        child
            .stdin
            .as_ref()
            .unwrap()
            .write_all(format!("!save,{}\n", path).as_bytes())
            .unwrap();

        let mut error = String::new();
        child
            .stderr
            .as_mut()
            .unwrap()
            .read_to_string(&mut error)
            .ok();
        if !error.is_empty() {
            return Err(error);
        }

        let mut output = String::new();
        child
            .stdout
            .as_mut()
            .unwrap()
            .read_to_string(&mut output)
            .ok();
        Ok(output)
    }
}

impl<const N: usize, T> Savable for RTPlotter<N, T>
where
    T: Real + ToString,
{
    fn save(&mut self, path: &str) -> Result<String, String> {
        let Some(child) = self.child.as_mut() else {
            return Err("Plotter process is not running.".to_string());
        };

        child
            .stdin
            .as_ref()
            .unwrap()
            .write_all(format!("!save,{}\n", path).as_bytes())
            .unwrap();

        let mut error = String::new();
        child
            .stderr
            .as_mut()
            .unwrap()
            .read_to_string(&mut error)
            .ok();
        if !error.is_empty() {
            return Err(error);
        }

        let mut output = String::new();
        child
            .stdout
            .as_mut()
            .unwrap()
            .read_to_string(&mut output)
            .ok();
        Ok(output)
    }
}

pub trait JoinAll {
    fn join_all(&mut self);
}

impl JoinAll for [Box<dyn Joinable>] {
    fn join_all(&mut self) {
        for plotter in self {
            plotter.join();
        }
    }
}

impl JoinAll for Vec<Box<dyn Joinable>> {
    fn join_all(&mut self) {
        for plotter in self {
            plotter.join();
        }
    }
}

impl<T, S> JoinAll for (T, S)
where
    T: Joinable,
    S: Joinable,
{
    fn join_all(&mut self) {
        self.0.join();
        self.1.join();
    }
}

impl<T, S, R> JoinAll for (T, S, R)
where
    T: Joinable,
    S: Joinable,
    R: Joinable,
{
    fn join_all(&mut self) {
        self.0.join();
        self.1.join();
        self.2.join();
    }
}
