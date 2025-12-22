use crate::{block::Block, signal::Signal, time::TimeType};
use alloc::vec::Vec;
use core::{
    iter::Sum,
    marker::PhantomData,
    ops::{Div, Mul, Sub},
};
use num_traits::{Signed, Zero};

#[derive(Debug, Clone, PartialEq)]
pub struct GoodHart<T, K>
where
    T: Zero
        + Signed
        + Copy
        + Div<f64, Output = T>
        + Sub<Output = T>
        + Mul<f64, Output = T>
        + Sum<T>,
    K: TimeType,
{
    error: Vec<T>,
    control_signal: Vec<T>,
    alphas: (f64, f64, f64),
    _marker: PhantomData<K>,
}

impl<T, K> GoodHart<T, K>
where
    T: Zero
        + Signed
        + Copy
        + Div<f64, Output = T>
        + Sub<Output = T>
        + Mul<f64, Output = T>
        + Sum<T>,
    K: TimeType,
{
    pub fn new(alpha1: f64, alpha2: f64, alpha3: f64) -> Self {
        Self {
            error: Vec::new(),
            control_signal: Vec::new(),
            alphas: (alpha1, alpha2, alpha3),
            _marker: PhantomData,
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

impl<T, K> Block for GoodHart<T, K>
where
    T: Zero
        + Signed
        + Copy
        + Div<f64, Output = T>
        + Sub<Output = T>
        + Mul<f64, Output = T>
        + Sum<T>,
    K: TimeType,
{
    type Input = (T, T);
    type Output = (T, T);
    type TimeType = K;

    fn output(
        &mut self,
        input: Signal<Self::Input, Self::TimeType>,
    ) -> Signal<Self::Output, Self::TimeType> {
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
