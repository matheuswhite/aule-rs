use crate::block::{AsInput, Input, Signal};
use core::time::Duration;

pub struct Setpoint {
    value: f32,
}

impl Setpoint {
    pub fn new(value: f32) -> Self {
        Setpoint { value }
    }
}

impl Input for Setpoint {
    fn output(&mut self, dt: Duration) -> Signal {
        Signal {
            value: self.value,
            dt,
        }
    }
}

impl AsInput for Setpoint {}
