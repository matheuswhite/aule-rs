use crate::block::{AsInput, Input, Signal};
use std::time::Duration;

pub struct Step {
    value: f32,
    sim_time: Duration,
}

impl Step {
    pub fn new() -> Self {
        Step {
            value: 1.0,
            sim_time: Duration::default(),
        }
    }

    pub fn with_value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }
}

impl Input for Step {
    fn output(&mut self, dt: Duration) -> Signal {
        self.sim_time += dt;

        if self.sim_time >= Duration::from_secs(1) {
            Signal {
                value: self.value,
                dt,
            }
        } else {
            Signal { value: 0.0, dt }
        }
    }
}

impl AsInput for Step {}
