use crate::{block::Block, signal::Signal};

#[derive(Debug, Clone, PartialEq)]
pub struct Ramp {
    value: f32,
}

impl Ramp {
    pub fn new(value: f32) -> Self {
        Ramp { value }
    }
}

impl Default for Ramp {
    fn default() -> Self {
        Self { value: 1.0 }
    }
}

impl Block for Ramp {
    type Input = ();
    type Output = f32;

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        let value = self.value * input.delta.sim_time().as_secs_f32();
        Signal {
            value,
            delta: input.delta,
        }
    }
}
