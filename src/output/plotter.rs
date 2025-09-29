use crate::output::Output;
use crate::signal::Signal;
use alloc::vec;
use alloc::vec::Vec;
use std::boxed::Box;
use std::format;
use std::io::{Read, Write};
use std::process::{Child, Command, Stdio};
use std::string::String;
use std::string::ToString;
use std::time::{Duration, Instant};

pub struct Plotter {
    sim_time: Duration,
    data: Vec<Vec<Signal<f32>>>,
    child: Option<Child>,
    grid: (f32, f32),
    title: String,
}

pub struct RTPlotter {
    sim_time: Duration,
    last_update: Instant,
    child: Option<Child>,
    grid: (f32, f32),
    title: String,
}

pub trait Joinable {
    fn join(&mut self);
}

pub trait Savable {
    fn save(&mut self, path: &str) -> Result<String, String>;
}

impl Plotter {
    pub fn new(title: String, x_grid: f32, y_grid: f32) -> Self {
        Self {
            sim_time: Duration::from_secs(0),
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
            let time = &signals[0].dt.as_secs_f32();

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

impl RTPlotter {
    pub fn new(title: String, x_grid: f32, y_grid: f32) -> Self {
        Self {
            sim_time: Duration::from_secs(0),
            last_update: Instant::now(),
            child: None,
            grid: (x_grid, y_grid),
            title,
        }
    }
}

impl Output<f32> for Plotter {
    fn show(&mut self, inputs: Signal<f32>) {
        self.sim_time += inputs.dt;
        self.data.push(vec![Signal {
            value: inputs.value,
            dt: self.sim_time.into(),
        }]);
    }
}

impl<const N: usize> Output<[f32; N]> for Plotter {
    fn show(&mut self, inputs: Signal<[f32; N]>) {
        self.sim_time += inputs.dt;
        self.data.push(
            inputs
                .value
                .iter()
                .map(|s| Signal {
                    value: *s,
                    dt: self.sim_time.into(),
                })
                .collect(),
        );
    }
}

impl Output<f32> for RTPlotter {
    fn show(&mut self, inputs: Signal<f32>) {
        self.sim_time += inputs.dt;

        if Instant::now().duration_since(self.last_update) < Duration::from_millis(17) {
            return;
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
            .write_all(format!("{},{}\n", self.sim_time.as_secs_f32(), inputs.value).as_bytes())
            .unwrap();
    }
}

impl<const N: usize> Output<[f32; N]> for RTPlotter {
    fn show(&mut self, inputs: Signal<[f32; N]>) {
        self.sim_time += inputs.dt;

        if Instant::now().duration_since(self.last_update) < Duration::from_millis(17) {
            return;
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
                    self.sim_time.as_secs_f32(),
                    inputs
                        .value
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                )
                .as_bytes(),
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

impl Savable for Plotter {
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

impl Savable for RTPlotter {
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
