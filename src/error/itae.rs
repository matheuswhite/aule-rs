use crate::{
    error::{AsErrorMetric, ErrorMetric},
    signal::Signal,
};

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

impl ErrorMetric<1> for ITAE {
    fn update(&mut self, input: [Signal; 1]) -> [Signal; 1] {
        self.n += 1;
        self.acc += self.n as f32 * input[0].value.abs();
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

impl AsErrorMetric<1> for ITAE {}
