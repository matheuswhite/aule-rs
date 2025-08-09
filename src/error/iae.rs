use crate::{
    error::{AsErrorMetric, ErrorMetric},
    signal::Signal,
};

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

impl ErrorMetric<1> for IAE {
    fn update(&mut self, input: [Signal; 1]) -> [Signal; 1] {
        self.acc += input[0].value.abs();
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

impl AsErrorMetric<1> for IAE {}
