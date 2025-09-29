use crate::{metrics::Metric, signal::Signal};

#[derive(Debug, Clone, PartialEq)]
pub struct ITAE {
    acc: f32,
    n: usize,
}

impl ITAE {
    pub fn new() -> Self {
        ITAE { acc: 0.0, n: 0 }
    }
}

impl Metric for ITAE {
    type Input = f32;

    fn update(&mut self, input: Signal<Self::Input>) -> Signal<Self::Input> {
        self.n += 1;
        self.acc += self.n as f32 * input.value.abs();
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
