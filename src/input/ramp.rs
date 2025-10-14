use crate::{input::Input, signal::Signal};
use core::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub struct Ramp {
    value: f32,
    sim_time: Duration,
}

impl Ramp {
    pub fn new(value: f32) -> Self {
        Ramp {
            value,
            sim_time: Duration::default(),
        }
    }
}

impl Default for Ramp {
    fn default() -> Self {
        Self {
            value: 1.0,
            sim_time: Default::default(),
        }
    }
}

impl Input for Ramp {
    type Output = f32;

    fn output(&mut self, dt: Duration) -> Signal<Self::Output> {
        self.sim_time += dt;
        let value = self.value.clone() * self.sim_time.as_secs_f32();
        Signal { value, dt }
    }
}
