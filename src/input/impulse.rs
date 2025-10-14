use crate::{input::Input, signal::Signal};
use core::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub struct Impulse {
    value: Option<f32>,
}

impl Impulse {
    pub fn new(value: f32) -> Self {
        Impulse { value: Some(value) }
    }
}

impl Default for Impulse {
    fn default() -> Self {
        Self {
            value: Some(f32::MAX),
        }
    }
}

impl Input for Impulse {
    type Output = f32;

    fn output(&mut self, dt: Duration) -> Signal<Self::Output> {
        match self.value.take() {
            Some(value) => {
                self.value = None; // Reset value after output
                Signal { value, dt }
            }
            None => Signal { value: 0.0, dt }, // If no value is set, return 0.0
        }
    }
}
