use crate::{metrics::Metric, signal::Signal};
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct GoodHart {
    error: Vec<f32>,
    control_signal: Vec<f32>,
    alphas: (f32, f32, f32),
}

impl GoodHart {
    pub fn new(alpha1: f32, alpha2: f32, alpha3: f32) -> Self {
        GoodHart {
            error: Vec::new(),
            control_signal: Vec::new(),
            alphas: (alpha1, alpha2, alpha3),
        }
    }
}

impl Metric for GoodHart {
    type Input = [f32; 2];

    fn update(&mut self, input: Signal<Self::Input>) -> Signal<Self::Input> {
        let error = input.value[0];
        let control_signal = input.value[1];
        self.error.push(error);
        self.control_signal.push(control_signal);

        input
    }

    fn value(&self) -> f32 {
        if self.error.is_empty() || self.control_signal.is_empty() {
            return 0.0;
        }

        let n = self.error.len() as f32;

        let e1 = self.control_signal.iter().sum::<f32>() / n;
        let e2 = self.control_signal.iter().map(|u| u - e1).sum::<f32>() / n;
        let e3 = self.error.iter().map(|e| e.abs()).sum::<f32>() / n;

        self.alphas.0 * e1 + self.alphas.1 * e2 + self.alphas.2 * e3
    }
}
