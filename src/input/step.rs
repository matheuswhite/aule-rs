use crate::input::{Input, Signal};
use core::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
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
    type Output = f32;

    fn output(&mut self, dt: Duration) -> Signal<Self::Output> {
        Signal {
            value: self.value,
            dt,
        }
    }
}
