use crate::block::Block;
use crate::signal::Signal;
use alloc::vec::Vec;
use core::marker::PhantomData;
use std::boxed::Box;
use std::format;
use std::io::{Read, Write};
use std::process::{Child, Command, Stdio};
use std::string::String;
use std::string::ToString;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Plotter<const N: usize> {
    data: Vec<[Signal<f32>; N]>,
    child: Option<Child>,
    grid: (f32, f32),
    title: String,
}

#[derive(Debug)]
pub struct RTPlotter<const N: usize> {
    last_update: Instant,
    child: Option<Child>,
    grid: (f32, f32),
    title: String,
    _marker: PhantomData<[(); N]>,
}

pub trait Joinable {
    fn join(&mut self);
}

pub trait Savable {
    fn save(&mut self, path: &str) -> Result<String, String>;
}

impl<const N: usize> Plotter<N> {
    pub fn new(title: String, x_grid: f32, y_grid: f32) -> Self {
        Self {
            data: Vec::new(),
            child: None,
            grid: (x_grid, y_grid),
            title,
        }
    }

    pub fn display(&mut self) {
        self.child = Some(
            Command::new("rtgraph")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .arg("-xs")
                .arg(self.grid.0.to_string())
                .arg("-ys")
                .arg(self.grid.1.to_string())
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

impl<const N: usize> RTPlotter<N> {
    pub fn new(title: String, x_grid: f32, y_grid: f32) -> Self {
        Self {
            last_update: Instant::now(),
            child: None,
            grid: (x_grid, y_grid),
            title,
            _marker: PhantomData,
        }
    }
}

impl<const N: usize> Block for Plotter<N> {
    type Input = [f32; N];
    type Output = [f32; N];

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        self.data.push(input.value.map(|s| Signal {
            value: s,
            delta: input.delta,
        }));
        input
    }
}

impl<const N: usize> Block for RTPlotter<N> {
    type Input = [f32; N];
    type Output = [f32; N];

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        if Instant::now().duration_since(self.last_update) < Duration::from_millis(17) {
            return input;
        }
        self.last_update = Instant::now();

        if self.child.is_none() {
            let command = Command::new("rtgraph")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .arg("-xs")
                .arg(self.grid.0.to_string())
                .arg("-ys")
                .arg(self.grid.1.to_string())
                .arg("-t")
                .arg(&self.title)
                .spawn()
                .expect("Failed to start rtgraph process");
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
}

impl<const N: usize> Drop for Plotter<N> {
    fn drop(&mut self) {
        if let Some(child) = &mut self.child {
            child.kill().unwrap();
        }
    }
}

impl<const N: usize> Drop for RTPlotter<N> {
    fn drop(&mut self) {
        if let Some(child) = &mut self.child {
            child.kill().unwrap();
        }
    }
}

impl<const N: usize> Joinable for Plotter<N> {
    fn join(&mut self) {
        if let Some(child) = &mut self.child {
            let _ = child.wait();
        }
    }
}

impl<const N: usize> Joinable for RTPlotter<N> {
    fn join(&mut self) {
        if let Some(child) = &mut self.child {
            let _ = child.wait();
        }
    }
}

impl<const N: usize> Savable for Plotter<N> {
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
        let _ = child.stderr.as_mut().unwrap().read_to_string(&mut error);
        if !error.is_empty() {
            return Err(error);
        }

        let mut output = String::new();
        let _ = child.stdout.as_mut().unwrap().read_to_string(&mut output);
        Ok(output)
    }
}

impl<const N: usize> Savable for RTPlotter<N> {
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
        let _ = child.stderr.as_mut().unwrap().read_to_string(&mut error);
        if !error.is_empty() {
            return Err(error);
        }

        let mut output = String::new();
        let _ = child.stdout.as_mut().unwrap().read_to_string(&mut output);
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
