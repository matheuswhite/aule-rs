use crate::block::{AsBlock, Block, Signal};

pub struct Gain {
    value: f32,
}

impl Gain {
    pub fn new(value: f32) -> Self {
        Gain { value }
    }
}

impl Block for Gain {
    fn output(&mut self, input: Signal) -> Signal {
        Signal {
            value: input.value * self.value,
            dt: input.dt,
        }
    }

    fn last_output(&self) -> Option<Signal> {
        None
    }
}

impl AsBlock for Gain {}
