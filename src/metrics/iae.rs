use crate::{metrics::Metric, signal::Signal};

#[derive(Debug, Clone, PartialEq)]
pub struct IAE {
    acc: f32,
    n: usize,
}

impl IAE {
    pub fn new() -> Self {
        IAE { acc: 0.0, n: 0 }
    }
}

impl Metric for IAE {
    type Input = f32;

    fn update(&mut self, input: Signal<Self::Input>) -> Signal<Self::Input> {
        self.acc += input.value.abs();
        self.n += 1;
        input
    }

    fn value(&self) -> f32 {
        if self.n == 0 {
            0.0
        } else {
            self.acc / self.n as f32
        }
    }
}
