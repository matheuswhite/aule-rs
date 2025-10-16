use crate::{block::Block, signal::Signal};

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

impl Block for Impulse {
    type Input = ();
    type Output = f32;

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        match self.value.take() {
            Some(value) => {
                self.value = None; // Reset value after output
                Signal {
                    value,
                    delta: input.delta,
                }
            }
            None => Signal {
                value: 0.0,
                delta: input.delta,
            }, // If no value is set, return 0.0
        }
    }
}
