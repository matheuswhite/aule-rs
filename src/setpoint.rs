use crate::block::{Input, Signal};
use core::time::Duration;

pub struct Setpoint {
    value: f32,
    max_time: Option<Duration>,
    sim_time: Duration,
    dt: Duration,
}

impl Setpoint {
    pub fn new(value: f32, dt: Duration) -> Self {
        Setpoint {
            value,
            max_time: None,
            sim_time: Duration::default(),
            dt,
        }
    }

    pub fn with_max_time(mut self, max_time: Duration) -> Self {
        self.max_time = Some(max_time);
        self
    }
}

impl Input for Setpoint {}

impl Iterator for Setpoint {
    type Item = Signal;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(max_time) = self.max_time {
            if self.sim_time > max_time {
                return None;
            }

            self.sim_time += self.dt;
        }

        Some(Signal {
            value: self.value,
            dt: self.dt,
        })
    }
}
