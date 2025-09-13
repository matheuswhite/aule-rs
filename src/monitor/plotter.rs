use crate::monitor::{AsMonitor, Monitor};
use crate::signal::Signal;
use alloc::vec::Vec;
use std::boxed::Box;
use std::format;
use std::io::Write;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

pub struct Plotter {
    sim_time: Duration,
    data: Vec<Vec<Signal>>,
    child: Option<Child>,
}

pub struct RTPlotter {
    sim_time: Duration,
    last_update: Instant,
    child: Option<Child>,
}

pub trait Joinable {
    fn join(&mut self);
}

impl Plotter {
    pub fn new() -> Self {
        Self {
            sim_time: Duration::from_secs(0),
            data: Vec::new(),
            child: None,
        }
    }

    pub fn display(&mut self) {
        self.child = Some(
            Command::new("rtgraph")
                .stdin(Stdio::piped())
                .spawn()
                .unwrap(),
        );

        for signals in &self.data {
            let first_signal = &signals[0];
            let time = first_signal.dt.as_secs_f32();
            let value = first_signal.value;

            if let Some(child) = &self.child {
                child
                    .stdin
                    .as_ref()
                    .unwrap()
                    .write_all(format!("{},{}\n", time, value).as_bytes())
                    .unwrap();
            }
        }
    }
}

impl RTPlotter {
    pub fn new() -> Self {
        Self {
            sim_time: Duration::from_secs(0),
            last_update: Instant::now(),
            child: None,
        }
    }
}

impl Monitor for Plotter {
    fn show(&mut self, input: Vec<Signal>) {
        self.sim_time += input[0].dt;
        self.data.push(
            input
                .into_iter()
                .map(|s| Signal {
                    value: s.value,
                    dt: self.sim_time.into(),
                })
                .collect(),
        );
    }
}

impl Monitor for RTPlotter {
    fn show(&mut self, input: Vec<Signal>) {
        self.sim_time += input[0].dt;
        let first_signal = &input[0];

        if Instant::now().duration_since(self.last_update) < Duration::from_millis(17) {
            return;
        }
        self.last_update = Instant::now();

        if self.child.is_none() {
            let command = Command::new("rtgraph")
                .stdin(Stdio::piped())
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
                format!("{},{}\n", self.sim_time.as_secs_f32(), first_signal.value).as_bytes(),
            )
            .unwrap();
    }
}

impl Drop for Plotter {
    fn drop(&mut self) {
        if let Some(child) = &mut self.child {
            child.kill().unwrap();
        }
    }
}

impl Drop for RTPlotter {
    fn drop(&mut self) {
        if let Some(child) = &mut self.child {
            child.kill().unwrap();
        }
    }
}

impl Joinable for Plotter {
    fn join(&mut self) {
        if let Some(child) = &mut self.child {
            let _ = child.wait();
        }
    }
}

impl Joinable for RTPlotter {
    fn join(&mut self) {
        if let Some(child) = &mut self.child {
            let _ = child.wait();
        }
    }
}

impl AsMonitor for Plotter {}

impl AsMonitor for RTPlotter {}

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
