use crate::{
    metrics::{AsMetric, Metric},
    signal::Signal,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ISE {
    acc: f32,
    n: usize,
}

impl ISE {
    pub fn new() -> Self {
        ISE { acc: 0.0, n: 0 }
    }
}

impl Metric<1> for ISE {
    fn update(&mut self, input: [Signal; 1]) -> [Signal; 1] {
        self.acc += input[0].value * input[0].value;
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

impl AsMetric<1> for ISE {}
