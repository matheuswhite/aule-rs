use crate::block::siso::{AsSISO, SISO};
use crate::signal::Signal;

pub struct Saturation {
    min: f32,
    max: f32,
    last_output: Option<Signal>,
}

impl From<f32> for Saturation {
    fn from(value: f32) -> Self {
        Saturation {
            min: -value,
            max: value,
            last_output: None,
        }
    }
}

impl From<(f32, f32)> for Saturation {
    fn from((min, max): (f32, f32)) -> Self {
        Saturation {
            min,
            max,
            last_output: None,
        }
    }
}

impl SISO for Saturation {
    fn output(&mut self, input: Signal) -> Signal {
        let saturated_value = input.value.clamp(self.min, self.max);
        let output = Signal {
            value: saturated_value,
            dt: input.dt,
        };
        self.last_output = Some(output);
        output
    }

    fn last_output(&self) -> Option<Signal> {
        self.last_output
    }
}

impl AsSISO for Saturation {}
