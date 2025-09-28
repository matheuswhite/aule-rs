use crate::{
    input::{AsInput, Input},
    signal::Signal,
};
use core::time::Duration;

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

impl Input for Ramp {
    fn output(&mut self, dt: Duration) -> Signal {
        self.sim_time += dt;
        let value = self.value * self.sim_time.as_secs_f32();
        Signal { value, dt }
    }
}

impl AsInput for Ramp {}
