use crate::{block::Block, signal::Signal};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ISE {
    acc: f32,
    n: usize,
}

impl ISE {
    pub fn value(&self) -> f32 {
        if self.n == 0 {
            0.0
        } else {
            self.acc / self.n as f32
        }
    }
}

impl Block for ISE {
    type Input = f32;
    type Output = f32;

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        self.acc += input.value * input.value;
        self.n += 1;
        input
    }
}
