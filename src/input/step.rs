use crate::{block::Block, signal::Signal};

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

impl Block for Step {
    type Input = ();
    type Output = f32;

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        Signal {
            value: self.value,
            delta: input.delta,
        }
    }
}
