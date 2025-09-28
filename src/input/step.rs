use crate::input::{AsInput, Input, Signal};
use core::time::Duration;

pub struct Step {
    value: f32,
}

impl Step {
    pub fn new(value: f32) -> Self {
        Step { value }
    }
}

impl Default for Step {
    fn default() -> Self {
        Step { value: 1.0 }
    }
}

impl Input for Step {
    fn output(&mut self, dt: Duration) -> Signal {
        Signal {
            value: self.value,
            dt,
        }
    }
}

impl AsInput for Step {}
