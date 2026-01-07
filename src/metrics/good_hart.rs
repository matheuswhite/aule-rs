use crate::{block::Block, signal::Signal};
use alloc::vec::Vec;
use core::{
    iter::Sum,
    ops::{Div, Mul, Sub},
};
use num_traits::{Signed, Zero};

#[derive(Debug, Clone, PartialEq)]
pub struct GoodHart<T>
where
    T: Zero
        + Signed
        + Copy
        + Div<f64, Output = T>
        + Sub<Output = T>
        + Mul<f64, Output = T>
        + Sum<T>,
{
    error: Vec<T>,
    control_signal: Vec<T>,
    alphas: (f64, f64, f64),
}

impl<T> GoodHart<T>
where
    T: Zero
        + Signed
        + Copy
        + Div<f64, Output = T>
        + Sub<Output = T>
        + Mul<f64, Output = T>
        + Sum<T>,
{
    pub fn new(alpha1: f64, alpha2: f64, alpha3: f64) -> Self {
        Self {
            error: Vec::new(),
            control_signal: Vec::new(),
            alphas: (alpha1, alpha2, alpha3),
        }
    }

    pub fn value(&self) -> T {
        if self.error.is_empty() || self.control_signal.is_empty() {
            return T::zero();
        }

        let n = self.error.len() as f64;

        let e1 = self.control_signal.iter().cloned().sum::<T>() / n;
        let e2 = self.control_signal.iter().map(|u| *u - e1).sum::<T>() / n;
        let e3 = self.error.iter().map(|e| e.abs()).sum::<T>() / n;

        e1 * self.alphas.0 + e2 * self.alphas.1 + e3 * self.alphas.2
    }
}

impl<T> Block for GoodHart<T>
where
    T: Zero
        + Signed
        + Copy
        + Div<f64, Output = T>
        + Sub<Output = T>
        + Mul<f64, Output = T>
        + Sum<T>,
{
    type Input = (T, T);
    type Output = (T, T);

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        let error = input.value.0;
        let control_signal = input.value.1;
        self.error.push(error);
        self.control_signal.push(control_signal);

        input
    }

    fn reset(&mut self) {
        self.error.clear();
        self.control_signal.clear();
    }
}
