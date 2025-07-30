use crate::block::Input;
use std::time::Duration;

pub struct Step {
    value: f32,
    max_time: Option<Duration>,
    sim_time: Duration,
    dt: Duration,
}

impl Step {
    pub fn new(dt: Duration) -> Self {
        Step {
            value: 1.0,
            max_time: None,
            sim_time: Duration::default(),
            dt,
        }
    }

    pub fn with_value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn with_max_time(mut self, max_time: Duration) -> Self {
        self.max_time = Some(max_time);
        self
    }
}

impl Input for Step {}

impl Iterator for Step {
    type Item = crate::block::Signal;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(max_time) = self.max_time {
            if self.sim_time >= max_time {
                return None;
            }
        }

        self.sim_time += self.dt;

        if self.sim_time >= Duration::from_secs(1) {
            Some(crate::block::Signal {
                value: self.value,
                dt: self.dt,
            })
        } else {
            Some(crate::block::Signal {
                value: 0.0,
                dt: self.dt,
            })
        }
    }
}
