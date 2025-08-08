use crate::{
    error::{AsErrorMetric, ErrorMetric},
    signal::Signal,
};

pub struct GoodHart {
    error: Vec<f32>,
    control_signal: Vec<f32>,
    alphas: [f32; 3],
}

impl GoodHart {
    pub fn new(alpha1: f32, alpha2: f32, alpha3: f32) -> Self {
        GoodHart {
            error: Vec::new(),
            control_signal: Vec::new(),
            alphas: [alpha1, alpha2, alpha3],
        }
    }
}

impl ErrorMetric<2> for GoodHart {
    fn update(&mut self, input: [Signal; 2]) -> [Signal; 2] {
        let error = input[0].value;
        let control_signal = input[1].value;
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

        self.alphas[0] * e1 + self.alphas[1] * e2 + self.alphas[2] * e3
    }
}

impl AsErrorMetric<2> for GoodHart {}
