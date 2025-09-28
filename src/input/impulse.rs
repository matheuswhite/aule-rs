use crate::{
    input::{AsInput, Input},
    signal::Signal,
};
use core::time::Duration;

pub struct Impulse {
    value: Option<f32>,
}

impl Impulse {
    pub fn new(value: f32) -> Self {
        Impulse { value: Some(value) }
    }
}

impl Input for Impulse {
    fn output(&mut self, dt: Duration) -> Signal {
        match self.value.take() {
            Some(value) => {
                self.value = None; // Reset value after output
                Signal { value, dt }
            }
            None => Signal { value: 0.0, dt }, // If no value is set, return 0.0
        }
    }
}

impl AsInput for Impulse {}
